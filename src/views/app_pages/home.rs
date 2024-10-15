use axum::response::{Html, IntoResponse};
use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "app_pages/home.stpl")]
struct AppHomePage {}

pub async fn home_page() -> impl IntoResponse {
    let ctx = AppHomePage {};

    Html(ctx.render_once().unwrap())
}
