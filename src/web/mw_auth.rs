use crate::ctx::Ctx;
use crate::JwtClaims;
use crate::{web::AUTH_TOKEN, Error, Result};

use async_trait::async_trait;
use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum::RequestPartsExt;
use axum::{http::Request, middleware::Next, response::Response};
use jsonwebtoken::{decode, DecodingKey, Validation};
use lazy_regex::regex_captures;
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
        println!("->> {:<12} - Ctx", "EXTRACTOR");

        let cookies = parts.extract::<Cookies>().await.unwrap();
        let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

        let (user_id, exp, sign) = auth_token
            .ok_or(Error::AuthFailNoAuthTokenCookie)
            .and_then(parse_token)?;

        Ok(Ctx::new(user_id))
    }
}

fn parse_token(token: String) -> Result<(String, String, String)> {
    let decoding_key = DecodingKey::from_secret("secret".as_ref());
    let token_data = decode::<JwtClaims>(&token, &decoding_key, &Validation::default());

    if let Ok(token) = token_data {
        // Retrieve the username from the JWT claims
        let username = token.claims.username;

        // Use the username as needed
        println!("Username: {}", username);
        Ok((username, "".to_owned(), "".to_owned()))
    } else {
        Err(Error::AuthFailTokenWrongFormat)
    }
}
