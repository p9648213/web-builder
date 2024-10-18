use axum::response::{Html, IntoResponse};
use sailfish::TemplateOnce;

use crate::utilities::minify::minify_html;

#[derive(TemplateOnce)]
#[template(path = "app/home.stpl")]
struct AppHomePage {}

pub async fn home_page() -> impl IntoResponse {
    let ctx = AppHomePage {};

    let html = ctx.render_once().unwrap();

    Html(minify_html(&html))
}
