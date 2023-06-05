use crate::ctx::Ctx;
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use sailfish::TemplateOnce;
use serde::{Deserialize, Serialize};

pub fn routes() -> Router {
    Router::new().route("/user", get(handler_user))
}

async fn handler_user(ctx: Ctx) -> impl IntoResponse {
    let name = &ctx.username();
    let body = Greet { name }.render_once().unwrap();
    Html(body)
}

#[derive(Debug, Serialize, Deserialize, TemplateOnce)]
#[template(path = "user.stpl")]
struct Greet<'a> {
    name: &'a str,
}
