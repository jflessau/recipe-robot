use leptos::{server, ServerFnError};

#[cfg(feature = "ssr")]
use crate::prelude::*;
#[cfg(feature = "ssr")]
use crate::prelude::*;
#[cfg(feature = "ssr")]
use crate::AppState;
#[cfg(feature = "ssr")]
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
#[cfg(feature = "ssr")]
use leptos_axum::{redirect, ResponseOptions};

#[cfg(feature = "ssr")]
const TOKEN_AGE_IN_DAYS: i64 = 7;

#[cfg(feature = "ssr")]
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
}

#[cfg(feature = "ssr")]
impl Claims {
    pub fn new(sub: Uuid) -> Self {
        Claims {
            sub,
            exp: (Utc::now() + Duration::days(TOKEN_AGE_IN_DAYS)).timestamp_millis() as usize,
        }
    }
}

#[server]
pub async fn join(_invite_code: String) -> Result<String, ServerFnError> {
    let invite_code_ok = true; // TODO: check invite code

    if !invite_code_ok {
        return Err(ServerFnError::new("Der Einladungscode ist nicht gültig."));
    }

    Ok("password".to_string())
}

#[server]
pub async fn login(_password: String) -> Result<(), ServerFnError> {
    let response = expect_context::<ResponseOptions>();
    let state = expect_context::<AppState>();

    let password_ok = true; // TODO: check password

    if !password_ok {
        return Err(ServerFnError::new("Das Passwort ist falsch."));
    }

    let max_age = 3600 * 24 * TOKEN_AGE_IN_DAYS;

    info!("encoding jwt");
    let jwt = encode(
        &Header::default(),
        &Claims::new(Uuid::new_v4()), // TODO
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
