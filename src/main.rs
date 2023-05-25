use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use sailfish::TemplateOnce;

pub use self::error::{Error, Result};

mod error;

#[derive(TemplateOnce)]
#[template(path = "index.stpl")]
struct Greet;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler_index));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler_index() -> impl IntoResponse {
    let body = Greet.render_once().unwrap();
    Html(body)
}
