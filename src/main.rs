use axum::{
    extract::Extension,
    http::{header, HeaderValue, StatusCode},
    middleware,
    response::{Html, IntoResponse, Json, Response},
    routing::get,
    Router,
};
use ctx::Ctx;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use sailfish::TemplateOnce;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::time::{Duration, SystemTime};

use tower_cookies::CookieManagerLayer;
use tower_cookies::{Cookie, Cookies};

pub use self::error::{Error, Result};

mod ctx;
mod web;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    username: String,
    // ... add other user fields as needed
}

#[derive(Debug, Serialize, Deserialize)]
struct JwtClaims {
    username: String,
    exp: usize,
    // ... add other claims as needed
}

mod error;

impl JwtClaims {
    fn new(username: &str) -> Self {
        let expiration = SystemTime::now()
            .checked_add(Duration::from_secs(3600)) // JWT expires in 1 hour
            .expect("Failed to calculate expiration time");

        JwtClaims {
            username: username.to_owned(),
            exp: expiration
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs() as usize,
        }
    }
}

#[derive(TemplateOnce)]
#[template(path = "index.stpl")]
struct Greet;

#[derive(TemplateOnce)]
#[template(path = "home.stpl")]
struct Home<'a> {
    name: &'a str,
}

//#region main

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler_index))
        .route("/login", get(handler_login))
        .route("/home", get(handler_home))
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth))
        .layer(CookieManagerLayer::new());

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

//#endregion

async fn handler_login(cookies: Cookies) -> Result<Json<Value>> {
    // Simulating user authentication
    let user = User {
        username: "john_doe".to_owned(),
    };
    // Generate a JWT token for the authenticated user
    let jwt_token = encode(
        &Header::default(),
        &JwtClaims::new(&user.username),
        &EncodingKey::from_secret("secret".as_ref()),
    )
    .unwrap();

    // Return the JWT token as a response
    cookies.add(Cookie::new("auth-token", jwt_token));
    let body = Json(json!({
        "result": {
            "success": true,
        }
    }));

    Ok(body)
}

async fn handler_index() -> impl IntoResponse {
    let body = Greet.render_once().unwrap();
    Html(body)
}

async fn handler_home(ctx: Ctx) -> impl IntoResponse {
    let name = &ctx.username();

    let body = Home { name }.render_once().unwrap();
    Html(body)
}
