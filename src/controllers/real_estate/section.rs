use axum::{
    extract::{Path, Query},
    response::{Html, IntoResponse},
};
use maud::html;

use crate::{
    models::error::AppError,
    views::real_estate::{home, property_details, search_result, shared},
};

use super::pages::PropertyQuery;

pub async fn get_section(
    Path(section): Path<String>,
    property_query: Query<PropertyQuery>,
) -> Result<impl IntoResponse, AppError> {
    match section.as_str() {
        "home" => {
            let html = html! {
                (home::render_home_banner())
                (home::render_home_search_box())
                (home::render_hot_properties())
                (home::render_our_services())
                (home::render_testimonial())
                (shared::render_contact())
            };

            Ok(Html(html.into_string()).into_response())
        }
        "search-results" => {
            let html = html! {
                (search_result::render_search_box())
                (search_result::render_search_result(None))
                (shared::render_contact())
            };

            Ok(Html(html.into_string()).into_response())
        }
        "property" => {
            let html = html! {
                (property_details::render_property_details(property_query.0))
                (shared::render_contact())
            };

            Ok(Html(html.into_string()).into_response())
        }
        _ => Ok(Html("Not found".to_owned()).into_response()),
    }
}
