use axum::response::{Html, IntoResponse};
use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "pages/login.stpl")]
struct LoginPageTemplate {}

pub async fn login_page() -> impl IntoResponse {
    let ctx = LoginPageTemplate {};
    Html(ctx.render_once().unwrap())
}

#[derive(TemplateOnce)]
#[template(path = "pages/register.stpl")]
struct RegisterPageTemplate {}

pub async fn register_page() -> impl IntoResponse {
    let ctx = RegisterPageTemplate {};
    Html(ctx.render_once().unwrap())
}
