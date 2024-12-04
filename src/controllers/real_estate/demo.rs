use axum::response::Html;

use crate::views::real_estate::home::render_home_page;

pub async fn get_real_estate_demo_page() -> Html<String> {
    Html(render_home_page().into_string())
}
