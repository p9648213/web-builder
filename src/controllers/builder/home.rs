use axum::response::Html;

use crate::views::builder::home::render_home_page;

pub async fn get_home_page() -> Html<String> {
    Html(render_home_page().into_string())
}
