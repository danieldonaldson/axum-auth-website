use std::time::Instant;

use aws_sdk_dynamodb::Client;
use axum::Extension;
use axum::{extract::Query, response::Json, routing::get, Router};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

use crate::app_config::AppConfig;
use crate::controllers::db::dynamo::get_user_by_email;
use crate::error::{Error, Result};
use crate::jwt_claims::JwtClaims;
use crate::mw::AUTH_TOKEN;

use crate::api::helpers::password::{self, hash_password};

pub fn routes() -> Router {
    Router::new()
        .route("/login", get(handler_login))
        .route("/logout", get(handler_logout))
}

async fn handler_login(
    cookies: Cookies,
    Extension(config): Extension<AppConfig>,
    Extension(db_client): Extension<Client>,
    Query(params): Query<LoginPayload>,
) -> Result<Json<Value>> {
    let start = Instant::now();
    // user authentication
    // dbg!(&params);
    //Check password
    if let Some(payload_username) = params.username {
        let user = get_user_by_email(db_client, payload_username).await?;
        if let Some(user) = user {
            if let Some(payload_password) = params.password {
                let hashed_password = hash_password(payload_password);
                dbg!(&hashed_password);
                if hashed_password != user.password {
                    let duration = start.elapsed();
                    println!(
                        "Time elapsed in expensive_function() is: {:?}",
                        duration
                    );
                    return Err(Error::AuthFailIncorrectPassword);
                }
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
                println!(
                    "Time elapsed in expensive_function() is: {:?}",
                    duration
                );
                Ok(body)
            } else {
                Err(Error::QueryFailNoPassword)
            }
        } else {
            Err(Error::QueryFailNoUsername)
        }
    } else {
        Err(Error::AuthFailUserNotFound)
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

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: Option<String>,
    password: Option<String>,
}
