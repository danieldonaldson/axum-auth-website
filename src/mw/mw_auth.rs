use std::time::{SystemTime, UNIX_EPOCH};

use crate::app_config::AppConfig;
use crate::ctx::Ctx;
use crate::jwt_claims::JwtClaims;
use crate::{mw::AUTH_TOKEN, Error, Result};

use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::RequestPartsExt;
use axum::{http::Request, middleware::Next, response::Response};
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
        let config = parts
            .extensions
            .get::<AppConfig>()
            .cloned()
            .ok_or(())
            .unwrap();
        let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

        let username = auth_token
            .ok_or(Error::AuthFailNoAuthTokenCookie)
            .and_then(|token| parse_token(token, config.jwt_secret_key))?;

        Ok(Ctx::new(username))
    }
}

fn parse_token(token: String, secret: String) -> Result<String> {
    let decoding_key = DecodingKey::from_secret(secret.as_ref());
    let token_data =
        decode::<JwtClaims>(&token, &decoding_key, &Validation::default());

    if let Ok(token) = token_data {
        let expiration_time = token.claims.exp;

        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        if expiration_time < current_time {
            return Err(Error::AuthFailTokenExpired);
        }

        // Retrieve the username from the JWT claims
        let username = token.claims.username;

        // Use the username as needed
        // println!("Username: {}", username);
        Ok(username)
    } else {
        Err(Error::AuthFailTokenWrongFormat)
    }
}
