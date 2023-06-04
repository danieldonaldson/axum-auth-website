use std::time::Instant;

use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;
use axum::Extension;
use axum::{response::Json, routing::get, Router};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

use crate::app_config::AppConfig;
use crate::controllers::db::dynamo::get_user_by_email;
use crate::error::{Error, Result};
use crate::jwt_claims::JwtClaims;
use crate::mw::AUTH_TOKEN;
use crate::user::User;
use crate::Error::DBFailFieldNotFound;

pub fn routes() -> Router {
    Router::new()
        .route("/login", get(handler_login))
        .route("/logout", get(handler_logout))
}

async fn handler_login(
    cookies: Cookies,
    Extension(config): Extension<AppConfig>,
    Extension(db_client): Extension<Client>,
) -> Result<Json<Value>> {
    let start = Instant::now();
    // user authentication
    if let Some(user) = get_user_by_email(db_client) {
        // Generate a JWT token for the authenticated user
        let jwt = encode(
            &Header::default(),
            &JwtClaims::new(&user.email),
            &EncodingKey::from_secret(config.jwt_secret_key.as_ref()),
        )
        .unwrap();

        // Return the JWT token as a response
        cookies.add(Cookie::new(AUTH_TOKEN, jwt));
        let body = Json(json!({
            "result": {
                "success": true,
            }
        }));

        let duration = start.elapsed();
        println!("Time elapsed in expensive_function() is: {:?}", duration);
        Ok(body)
    } else {
        return Err(Error::AuthFailUserNotFound);
    }
}

async fn handler_logout(cookies: Cookies) -> Result<Json<Value>> {
    // Return the JWT token as a response
    cookies.remove(Cookie::new(AUTH_TOKEN, ""));
    let body = Json(json!({
        "result": {
            "success": true,
        }
    }));

    Ok(body)
}
