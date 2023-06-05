use crate::ctx::Ctx;
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use sailfish::TemplateOnce;
use serde::{Deserialize, Serialize};

pub fn routes() -> Router {
    Router::new().route("/parent", get(handler_parent))
}

async fn handler_parent(ctx: Ctx) -> impl IntoResponse {
    let _name = &ctx.username();
    let body = Greet { _name }.render_once().unwrap();
    Html(body)
}

#[derive(Debug, Serialize, Deserialize, TemplateOnce)]
#[template(path = "parent.stpl")]
struct Greet<'a> {
    _name: &'a str,
}
