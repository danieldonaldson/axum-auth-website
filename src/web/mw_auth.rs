use std::time::{SystemTime, UNIX_EPOCH};

use crate::ctx::Ctx;
use crate::jwt_claims::JwtClaims;
use crate::{
    web::{AUTH_TOKEN, SECRET_KEY},
    Error, Result,
};

use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::RequestPartsExt;
use axum::{http::Request, middleware::Next, response::Response};
use chrono::{DateTime, NaiveDateTime, Utc};
use jsonwebtoken::{decode, DecodingKey, Validation};
use tower_cookies::Cookies;

pub async fn mw_require_auth<B>(
    cookies: Cookies,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    auth_token.ok_or(Error::AuthFailNoAuthTokenCookie)?;

    Ok(next.run(req).await)
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        // println!("->> {:<12} - Ctx", "EXTRACTOR");

        let cookies = parts.extract::<Cookies>().await.unwrap();
        let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

        let username = auth_token
            .ok_or(Error::AuthFailNoAuthTokenCookie)
            .and_then(parse_token)?;

        Ok(Ctx::new(username))
    }
}

fn parse_token(token: String) -> Result<String> {
    let decoding_key = DecodingKey::from_secret(SECRET_KEY.as_ref());
    let token_data =
        decode::<JwtClaims>(&token, &decoding_key, &Validation::default());

    if let Ok(token) = token_data {
        let expiration_time = token.claims.exp;

        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("SystemTime before UNIX EPOCH!")
            .as_secs();

        println!(
            "Expires at {}, time is now {} ",
            format_seconds_since_epoch(expiration_time),
            format_seconds_since_epoch(current_time)
        );
        if expiration_time < current_time {
            return Err(Error::AuthFailTokenExpired);
        }

        // Retrieve the username from the JWT claims
        let username = token.claims.username;

        // Use the username as needed
        println!("Username: {}", username);
        Ok(username)
    } else {
        Err(Error::AuthFailTokenWrongFormat)
    }
}

fn format_seconds_since_epoch(seconds: u64) -> String {
    let naive_datetime =
        NaiveDateTime::from_timestamp_opt(seconds as i64, 0).unwrap();
    let datetime: DateTime<Utc> = DateTime::from_utc(naive_datetime, Utc);
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}
