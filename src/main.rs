use axum::{
    middleware,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use sailfish::TemplateOnce;
use serde::{Deserialize, Serialize};
use tower_cookies::CookieManagerLayer;

use self::error::{Error, Result};
// use self::user::User;

mod ctx;
mod error;
mod jwt_claims;
mod user;
mod web;

#[derive(Debug, Serialize, Deserialize, TemplateOnce)]
#[template(path = "index.stpl")]
struct Greet;

//#region main

#[tokio::main]
async fn main() {
    let routes_login = web::routes_login::routes();
    let routes_protected =
        web::routes_home::routes().route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    let app = Router::new()
        .route("/hello", get(handler_hello))
        .merge(routes_login)
        .merge(routes_protected)
        .layer(CookieManagerLayer::new());

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

//#endregion

async fn handler_hello() -> impl IntoResponse {
    let body = Greet.render_once().unwrap();
    Html(body)
}
