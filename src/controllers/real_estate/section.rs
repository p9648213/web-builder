use axum::{
    extract::Path,
    response::{Html, IntoResponse},
};
use maud::html;

use crate::{
    models::error::AppError,
    views::real_estate::{
        components::{
            render_contact, render_home_banner, render_home_search_box, render_hot_property,
            render_our_services, render_testimonial,
        },
        search_components,
    },
};

pub async fn get_section(Path(section): Path<String>) -> Result<impl IntoResponse, AppError> {
    match section.as_str() {
        "home" => {
            let html = html! {
                (render_home_banner())
                (render_home_search_box())
                (render_hot_property())
                (render_our_services())
                (render_testimonial())
                (render_contact())
            };

            Ok(Html(html.into_string()).into_response())
        }
        "search-result" => {
            let html = html! {
                (search_components::render_search_box())
            };

            Ok(Html(html.into_string()).into_response())
        }
        _ => Ok(Html("Not found".to_owned()).into_response()),
    }
}
