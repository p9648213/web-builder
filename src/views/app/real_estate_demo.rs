use axum::response::{Html, IntoResponse};
use sailfish::TemplateOnce;

use crate::utilities::minify::minify_html;

#[derive(TemplateOnce)]
#[template(path = "app/real_estate_demo.stpl")]
struct RealEsateDemo {}

pub async fn real_estate_demo() -> impl IntoResponse {
    let ctx = RealEsateDemo {};

    let html = ctx.render_once().unwrap();

    Html(minify_html(&html))
}
