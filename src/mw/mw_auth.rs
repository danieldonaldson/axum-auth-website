use std::time::{SystemTime, UNIX_EPOCH};

use crate::api::routes_login::create_jwt;
use crate::app_config::AppConfig;
use crate::ctx::Ctx;
use crate::jwt_claims::{JwtClaims, REFRESH_TIME_BEFORE_EXPIRY};
use crate::{mw::AUTH_TOKEN, Error, Result};

use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::RequestPartsExt;
use axum::{http::Request, middleware::Next, response::Response};
use jsonwebtoken::{decode, DecodingKey, Validation};
use tower_cookies::{Cookie, Cookies};

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

        let (username, expiry) = auth_token
            .ok_or(Error::AuthFailNoAuthTokenCookie)
            .and_then(|token| parse_token(token, &config.jwt_secret_key))?;

        // Check if refresh needed
        // Can move this out to a new middleware, but worth the extra step?
        let current_time = get_current_time();
        println!(
            "Token for {} expires in {} seconds.",
            username,
            expiry - current_time,
            // expiry - REFRESH_TIME_BEFORE_EXPIRY - current_time,
            // current_time
        );
        if expiry - REFRESH_TIME_BEFORE_EXPIRY < current_time {
            // generate new jwt
            // Add check against database to see if this username has been revoked?
            let jwt = create_jwt(&username, &config);
            cookies.add(Cookie::new(AUTH_TOKEN, jwt));
            // set it in the cookie
            println!("Token refreshed for {}", username);
        }

        Ok(Ctx::new(username))
    }
}

fn parse_token(token: String, secret: &String) -> Result<(String, u64)> {
    let decoding_key = DecodingKey::from_secret(secret.as_ref());
    let token_data =
        decode::<JwtClaims>(&token, &decoding_key, &Validation::default());

    if let Ok(token) = token_data {
        let expiration_time = token.claims.exp;

        let current_time = get_current_time();

        if expiration_time < current_time {
            return Err(Error::AuthFailTokenExpired);
        }

        // Retrieve the username from the JWT claims
        let username = token.claims.username;

        // Use the username as needed
        // println!("Username: {}", username);
        Ok((username, expiration_time))
    } else {
        Err(Error::AuthFailTokenWrongFormat)
    }
}

fn get_current_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
