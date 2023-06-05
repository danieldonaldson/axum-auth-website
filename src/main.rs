use std::{fs::File, io::Read};

use axum::{
    middleware,
    response::{Html, IntoResponse},
    routing::get,
    Extension, Router,
};
use sailfish::TemplateOnce;
use serde::{Deserialize, Serialize};
use tower_cookies::CookieManagerLayer;

use self::error::{Error, Result};
use app_config::AppConfig;
use controllers::db::dynamo;

mod api;
mod app_config;
mod controllers;
mod ctx;
mod error;
mod jwt_claims;
mod mw;
mod pages;
mod user;

pub const ENV_FILE: &str = "env.toml";

//#region main

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");

    let env_config = load_config();

    let db_client = dynamo::create_db_client().await;

    let api_login = api::routes_login::routes();
    let pages_protected = pages::routes_home::routes()
        .route_layer(middleware::from_fn(mw::mw_auth::mw_require_auth));

    let app = Router::new()
        .route("/hello", get(handler_hello))
        .merge(api_login)
        .merge(pages_protected)
        .layer(CookieManagerLayer::new())
        .layer(Extension(env_config))
        .layer(Extension(db_client));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

//#endregion

fn load_config() -> AppConfig {
    let mut file = File::open(ENV_FILE).unwrap_or_else(|err| {
        eprintln!("Failed to open config file: {}\n {}", ENV_FILE, err);
        std::process::exit(1);
    });
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap_or_else(|err| {
        eprintln!("Failed to open config file: {}\n {}", ENV_FILE, err);
        std::process::exit(1);
    });

    let config: AppConfig = toml::from_str(&contents).unwrap_or_else(|err| {
        eprintln!("Failed to parse config file: {}. Please check correct syntax for a TOML file. \n {}", ENV_FILE, err);
        std::process::exit(1);
    });

    config
}

#[derive(Debug, Serialize, Deserialize, TemplateOnce)]
#[template(path = "index.stpl")]
struct Greet;

async fn handler_hello() -> impl IntoResponse {
    let body = Greet.render_once().unwrap();
    Html(body)
}
