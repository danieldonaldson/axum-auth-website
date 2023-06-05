use std::{fs::File, io::Read};

use self::error::{Error, Result};
use app_config::AppConfig;
use axum::{
    middleware,
    response::{Html, IntoResponse},
    routing::get,
    Extension, Router,
};
use controllers::db::dynamo;
use sailfish::TemplateOnce;
use serde::{Deserialize, Serialize};
use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;

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
    let pages_protected = pages::routes_home::routes().route_layer(
        ServiceBuilder::new()
            .layer(middleware::from_fn(mw::mw_auth::mw_require_auth)),
    );
    let pages_user = api::routes_user::routes().layer(
        ServiceBuilder::new()
            .layer(middleware::from_fn(mw::mw_auth::mw_require_auth))
            .layer(middleware::from_fn(mw::mw_auth::mw_require_user_or_higher)),
    );
    let pages_parent = api::routes_parent::routes().layer(
        ServiceBuilder::new()
            .layer(middleware::from_fn(mw::mw_auth::mw_require_auth))
            .layer(middleware::from_fn(
                mw::mw_auth::mw_require_parent_or_higher,
            )),
    );

    let app = Router::new()
        .route("/hello", get(handler_hello))
        .merge(api_login)
        .merge(pages_protected)
        .merge(pages_user)
        .merge(pages_parent)
        .layer(CookieManagerLayer::new())
        .layer(Extension(env_config))
        .layer(Extension(db_client));
    // Static routes must be served via nginx

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
