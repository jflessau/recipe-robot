use crate::prelude::*;
use async_trait::async_trait;

const TOKEN_AGE_IN_DAYS: i64 = 3;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

impl Claims {
    pub fn new(sub: String) -> Self {
        Claims {
            sub,
            exp: (Utc::now() + Duration::days(TOKEN_AGE_IN_DAYS)).timestamp_millis() as usize,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignupIn {
    pub invite_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignupOut {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LonginIn {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Invite {
    initial_charges: usize,
    used_charges: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Me {
    username: String,
    percentage_of_daily_limit: u8,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthenticatedUser {
    pub username: String,
}

impl AuthenticatedUser {
    pub fn new(username: String) -> Self {
        AuthenticatedUser { username }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(req: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let jar: CookieJar = CookieJar::from_request_parts(req, state)
            .await
            .map_err(|_| Error::Unauthorized)?;

        let Some(cookie) = jar.get("token") else {
            return Err(Error::Unauthorized);
        };

        let jwt_secret = std::env::var("JWT_SECRET").expect("missing JWT_SECRET");
        match decode::<Claims>(
            cookie.value(),
            &DecodingKey::from_base64_secret(&jwt_secret).unwrap(),
            &Validation::default(),
        ) {
            Ok(token) => Ok(AuthenticatedUser::new(token.claims.sub)),
            Err(_) => Err(Error::Unauthorized),
        }
    }
}

pub async fn join(
    Extension(state): Extension<AppState>,
    Json(payload): Json<SignupIn>,
) -> Result<Json<SignupOut>, Error> {
    let Some(mut invite): Option<Invite> =
        state.db.select(("invite", &payload.invite_code)).await?
    else {
        return Err(Error::Forbidden("invalid invite code".to_string()));
    };

    if invite.used_charges == invite.initial_charges {
        return Err(Error::Forbidden("invalid invite code".to_string()));
    }

    invite.used_charges += 1;
    let Some(_invite) = state
        .db
        .upsert::<Option<Invite>>(("invite", &payload.invite_code))
        .content(invite.clone())
        .await?
    else {
        error!("failed to update invite");
        return Err(Error::InternalServer);
    };

    let password: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(24)
        .map(char::from)
        .collect();
    let password_hash = hash(&password, DEFAULT_COST)?;
    let rng = RNG::from(&Language::Fantasy);
    let username = rng.generate_name_by_count(3).to_lowercase();

    let user = User {
        id: ("user", username.clone()).into(),
        password_hash,
    };

    let Some(_user) = state
        .db
        .upsert::<Option<User>>(user.id.clone())
        .content(user.clone())
        .await?
    else {
        error!("failed to create user");
        return Err(Error::InternalServer);
    };

    let Some(_r) = state
        .db
        .insert::<Vec<Relation>>("spawns")
        .relation(Relation {
            r#in: thing(&format!("invite:{}", payload.invite_code))?,
            out: thing(&format!("user:{username}"))?,
        })
        .await?
        .first()
    else {
        error!("failed to create invite -> user relation");
        return Err(Error::InternalServer);
    };

    info!("🎉 {username} joined!");

    Ok(Json(SignupOut { username, password }))
}

pub async fn login(
    Extension(state): Extension<AppState>,
    jar: CookieJar,
    Json(payload): Json<LonginIn>,
) -> Result<CookieJar, Error> {
    let Some(user): Option<User> = state.db.select(("user", &payload.username)).await? else {
        return Err(Error::Forbidden("invalid username".to_string()));
    };

    if !verify(&payload.password, &user.password_hash)? {
        return Err(Error::Forbidden("invalid password".to_string()));
    }

    info!("🔑 logging in user: {}", user.id.key());

    let jwt = encode(
        &Header::default(),
        &Claims::new(user.id.key().to_string()),
        &EncodingKey::from_base64_secret(&state.jwt_secret).unwrap(),
    )?;

    let mut cookie: Cookie = Cookie::build(("token", jwt))
        .path("/")
        .secure(true)
        .http_only(true)
        .into();

    cookie.set_expires(OffsetDateTime::now_utc() + CookieDuration::days(TOKEN_AGE_IN_DAYS));

    Ok(jar.add(cookie))
}

pub async fn logout(
    authenticated_user: AuthenticatedUser,
    jar: CookieJar,
) -> Result<CookieJar, Error> {
    info!("👋 logging out user: {}", authenticated_user.username);

    let mut cookie: Cookie = Cookie::build(("token", ""))
        .path("/")
        .secure(true)
        .http_only(true)
        .into();

    cookie.set_expires(OffsetDateTime::now_utc() - CookieDuration::days(1));

    Ok(jar.add(cookie))
}

pub async fn me(
    authenticated_user: AuthenticatedUser,
    Extension(state): Extension<AppState>,
) -> Result<Json<Me>, Error> {
    let Some(user): Option<User> = state
        .db
        .select(("user", &authenticated_user.username))
        .await?
    else {
        return Err(Error::NotFound);
    };

    // user limit

    let user_limit = limits().user_daily;
    let cost = user_daily_cost(&state.db, &authenticated_user.username).await?;
    let user_percentage = if user_limit < 0.0001 {
        100
    } else if cost < 0.0001 {
        0
    } else {
        ((cost / user_limit) * 100.0).round() as u8
    };

    // application limit

    let app_limit = limits().application_daily;
    let app_cost = application_daily_cost(&state.db).await?;
    let app_percentage = if app_limit < 0.0001 {
        100
    } else if app_cost < 0.0001 {
        0
    } else {
        ((app_cost / app_limit) * 100.0).round() as u8
    };

    // limit = max of user and app

    let mut percentage = std::cmp::max(user_percentage, app_percentage);
    if percentage > 100 {
        percentage = 100;
    }

    let me = Me {
        username: user.id.key().to_string(),
        percentage_of_daily_limit: percentage,
    };

    Ok(Json(me))
}
