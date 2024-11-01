use axum::response::Html;

use crate::views::real_estate::real_estate_demo::render_real_estate_demo_page;

pub async fn get_real_estate_demo_page() -> Html<String> {
    Html(render_real_estate_demo_page())
}
