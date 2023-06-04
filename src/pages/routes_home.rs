use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use sailfish::TemplateOnce;

use ctx::Ctx;

use crate::ctx;

#[derive(TemplateOnce)]
#[template(path = "home.stpl")]
struct Home<'a> {
    name: &'a str,
}

pub fn routes() -> Router {
    Router::new().route("/", get(handler_home))
}

async fn handler_home(ctx: Ctx) -> impl IntoResponse {
    let name = &ctx.username();

    let body = Home { name }.render_once().unwrap();
    Html(body)
}
