use axum::response::Html;

use crate::views::real_estate::pages::{render_home_page, render_search_result_page};

pub async fn get_real_estate_home_page() -> Html<String> {
    Html(render_home_page().into_string())
}

pub async fn get_real_estate_search_result_page() -> Html<String> {
    Html(render_search_result_page().into_string())
}
