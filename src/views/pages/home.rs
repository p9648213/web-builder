use axum::response::{Html, IntoResponse};
use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "pages/home.stpl")]
struct HomePageTemplate {}

pub async fn home_page() -> impl IntoResponse {
    let ctx = HomePageTemplate {};
    Html(ctx.render_once().unwrap())
}
