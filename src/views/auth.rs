use axum::response::{Html, IntoResponse};
use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "login.stpl")]
pub struct LoginTemplate {}

pub async fn login() -> impl IntoResponse {
    let ctx = LoginTemplate {};
    Html(ctx.render_once().unwrap())
}
