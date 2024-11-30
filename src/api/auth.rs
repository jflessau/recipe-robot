use leptos::{server, ServerFnError};

#[cfg(feature = "ssr")]
use crate::prelude::*;
#[cfg(feature = "ssr")]
use crate::AppState;
#[cfg(feature = "ssr")]
use jsonwebtoken::{encode, EncodingKey, Header};
#[cfg(feature = "ssr")]
use leptos_axum::{redirect, ResponseOptions};

#[cfg(feature = "ssr")]
const TOKEN_AGE_IN_DAYS: i64 = 7;

#[cfg(feature = "ssr")]
use super::db::{AuthenticatedUser, User};

#[cfg(feature = "ssr")]
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[cfg(feature = "ssr")]
impl Claims {
    pub fn new(sub: String) -> Self {
        Claims {
            sub,
            exp: (Utc::now() + Duration::days(TOKEN_AGE_IN_DAYS)).timestamp_millis() as usize,
        }
    }
}

#[server]
pub async fn join(invite_code: String) -> Result<(String, String), ServerFnError> {
    let state = expect_context::<AppState>();

    match User::join(&state.db, invite_code).await {
        Ok((username, password)) => Ok((username, password)),
        Err(e) => {
            log::error!("Error joining: {:?}", e);
            Err(ServerFnError::new("Error joining"))
        }
    }
}

#[server]
pub async fn login(username: String, password: String) -> Result<(), ServerFnError> {
    let response = expect_context::<ResponseOptions>();
    let state = expect_context::<AppState>();

    match User::login(&state.db, username, password).await {
        Err(e) => {
            log::error!("Error logging in: {:?}", e);
            Err(ServerFnError::new("Error logging in"))
        }
        Ok(username) => {
            let max_age = 3600 * 24 * TOKEN_AGE_IN_DAYS;

            info!("encoding jwt");
            let jwt = encode(
                &Header::default(),
                &Claims::new(username),
                &EncodingKey::from_base64_secret(&state.jwt_secret).unwrap(),
            )?;

            response.insert_header(
                http::header::SET_COOKIE,
                http::HeaderValue::from_str(&format!(
                    "jwt={jwt}; Max-Age={max_age}; Secure; HttpOnly; Path=/"
                ))
                .expect("couldn't set user cookie"),
            );

            redirect("/");
            Ok(())
        }
    }
}

#[server]
pub async fn logout() -> Result<(), ServerFnError> {
    let response = expect_context::<ResponseOptions>();

    let max_age = 0;

    response.insert_header(
        http::header::SET_COOKIE,
        http::HeaderValue::from_str(&format!(
            "jwt=; Max-Age={max_age}; Secure; HttpOnly; Path=/"
        ))
        .expect("couldn't set user cookie"),
    );

    redirect("/login");

    Ok(())
}

#[server]
pub async fn authorized() -> Result<bool, ServerFnError> {
    let u = expect_context::<Option<AuthenticatedUser>>();

    Ok(u.is_some())
}
