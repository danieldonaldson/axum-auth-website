use axum::{response::Json, routing::get, Router};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

use crate::error::Result;
use crate::jwt_claims::JwtClaims;
use crate::user::User;
use crate::web::{AUTH_TOKEN, SECRET_KEY};

pub fn routes() -> Router {
    Router::new().route("/login", get(handler_login))
}

async fn handler_login(cookies: Cookies) -> Result<Json<Value>> {
    // Simulating user authentication
    let user = User {
        username: "john_doe".to_owned(),
    };
    // Generate a JWT token for the authenticated user
    let jwt = encode(
        &Header::default(),
        &JwtClaims::new(&user.username),
        &EncodingKey::from_secret(SECRET_KEY.as_ref()),
    )
    .unwrap();

    // Return the JWT token as a response
    cookies.add(Cookie::new(AUTH_TOKEN, jwt));
    let body = Json(json!({
        "result": {
            "success": true,
        }
    }));

    Ok(body)
}
