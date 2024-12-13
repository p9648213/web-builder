use axum::{extract::Query, response::Html};
use maud::html;
use reqwest::StatusCode;
use serde::Deserialize;

use crate::{
    models::error::AppError,
    views::real_estate::components::{render_selection_drop_down, render_selection_label},
};

#[derive(Deserialize)]
pub struct DemoQuery {
    pub demo: bool,
}

pub async fn get_listing_type(Query(query): Query<DemoQuery>) -> Result<Html<String>, AppError> {
    if query.demo {
        let listing_types = vec!["Resales", "New Development", "Short rental", "Long rental"];

        let html = html! {
            (render_selection_drop_down(listing_types, "Resales"))
            (render_selection_label("Resales", "listing-type-label"))
        }
        .into_string();

        Ok(Html(html))
    } else {
        Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Server Error",
        ))
    }
}
