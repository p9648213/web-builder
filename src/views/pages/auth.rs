use axum::response::{Html, IntoResponse};
use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "pages/login.stpl")]
struct LoginPageTemplate {}

pub async fn login_page() -> impl IntoResponse {
    let ctx = LoginPageTemplate {};
    Html(ctx.render_once().unwrap())
}
