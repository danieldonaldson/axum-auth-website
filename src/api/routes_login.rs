use aws_sdk_dynamodb::Client;
use axum::Extension;
use axum::{extract::Query, response::Json, routing::get, Router};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

use crate::app_config::AppConfig;
use crate::controllers::db::dynamo::{create_user, get_user_by_email};
use crate::error::{Error, Result};
use crate::jwt_claims::JwtClaims;
use crate::mw::AUTH_TOKEN;

use crate::api::helpers::password::hash_password;

pub fn routes() -> Router {
    Router::new()
        .route("/login", get(handler_login))
        .route("/logout", get(handler_logout))
        .route("/sign-up", get(handler_sign_up))
}

async fn handler_login(
    cookies: Cookies,
    Extension(config): Extension<AppConfig>,
    Extension(db_client): Extension<Client>,
    Query(params): Query<LoginPayload>,
) -> Result<Json<Value>> {
    if let Some(payload_username) = params.username {
        let user =
            get_user_by_email(&db_client, payload_username, &config).await?;
        if let Some(user) = user {
            if let Some(payload_password) = params.password {
                let (salt, password) = user.password.split_at(16);
                let (hashed_password, _) = // throwing away the salt because we already have it
                    hash_password(payload_password, Some(salt.to_string())); // do we need to offload this to a non-blocking thread?
                                                                             // println!("hash={}\nsalt={}", hashed_password, salt);
                if hashed_password != password {
                    return Err(Error::AuthFailIncorrectPassword);
                }
                let jwt = create_jwt(&user.email, &config, user.group);
                // Return the JWT token as a response
                cookies.add(Cookie::new(AUTH_TOKEN, jwt));
                let body = Json(json!({
                    "result": {
                        "success": true,
                    }
                }));

                Ok(body)
            } else {
                Err(Error::QueryFailNoPassword)
            }
        } else {
            Err(Error::AuthFailUserNotFound)
        }
    } else {
        Err(Error::QueryFailNoUsername)
    }
}

async fn handler_sign_up(
    cookies: Cookies,
    Extension(config): Extension<AppConfig>,
    Extension(db_client): Extension<Client>,
    Query(params): Query<LoginPayload>,
) -> Result<Json<Value>> {
    if let Some(payload_username) = params.username {
        let user =
            get_user_by_email(&db_client, payload_username.clone(), &config)
                .await?;
        if user.is_some() {
            Err(Error::AuthFailUserAlreadyExists)
        } else if let Some(payload_password) = params.password {
            let (hashed_password, salt) = hash_password(payload_password, None); // do we need to offload this to a non-blocking thread?
                                                                                 // println!("hash={}\nsalt={}", hashed_password, salt);
            let salted_hashed_password = format!("{}{}", salt, hashed_password);

            // todo: Write the user to the db
            if create_user(
                &db_client,
                payload_username.clone(),
                salted_hashed_password,
                &config,
            )
            .await
            .is_ok()
            {
                let jwt = create_jwt(&payload_username, &config, 0);
                // Return the JWT token as a response
                cookies.add(Cookie::new(AUTH_TOKEN, jwt));
                let body = Json(json!({
                    "result": {
                        "success": true,
                    }
                }));
                Ok(body)
            } else {
                Err(Error::DBFailedToCreateUser)
            }
        } else {
            Err(Error::QueryFailNoPassword)
        }
    } else {
        Err(Error::QueryFailNoUsername)
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

pub fn create_jwt(username: &str, config: &AppConfig, group: u8) -> String {
    encode(
        &Header::default(),
        &JwtClaims::new(username, group),
        &EncodingKey::from_secret(config.jwt_secret_key.as_ref()),
    )
    .unwrap()
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: Option<String>,
    password: Option<String>,
}
