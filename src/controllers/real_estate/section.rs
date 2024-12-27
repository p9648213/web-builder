use axum::{
    extract::Path,
    response::{Html, IntoResponse},
};
use maud::html;

use crate::{
    models::error::AppError,
    views::real_estate::{home, search_result, shared},
};

pub async fn get_section(Path(section): Path<String>) -> Result<impl IntoResponse, AppError> {
    match section.as_str() {
        "home" => {
            let html = html! {
                (home::render_home_banner())
                (home::render_home_search_box())
                (home::render_hot_property())
                (home::render_our_services())
                (home::render_testimonial())
                (shared::render_contact())
            };

            Ok(Html(html.into_string()).into_response())
        }
        "search-result" => {
            let html = html! {
                (search_result::render_search_box())
                (search_result::render_search_result())
                (shared::render_contact())
            };

            Ok(Html(html.into_string()).into_response())
        }
        _ => Ok(Html("Not found".to_owned()).into_response()),
    }
}
