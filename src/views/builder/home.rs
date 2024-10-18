use axum::response::{Html, IntoResponse};
use sailfish::TemplateOnce;

use crate::utilities::minify::minify_html;

#[derive(TemplateOnce)]
#[template(path = "builder/home.stpl")]
struct BuilderHomePageTemplate {}

pub async fn home_page() -> impl IntoResponse {
    let ctx = BuilderHomePageTemplate {};

    let html = ctx.render_once().unwrap();

    Html(minify_html(&html)).into_response()
}
