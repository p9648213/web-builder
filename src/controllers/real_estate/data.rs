use axum::{extract::Query, response::Html};
use maud::html;
use reqwest::StatusCode;
use serde::Deserialize;

use crate::{
    models::error::AppError,
    views::real_estate::components::{
        render_beds_baths_selection_drop_down, render_price_input, render_selection_drop_down,
        render_selection_label,
    },
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

pub async fn get_price() -> Result<Html<String>, AppError> {
    Ok(Html(render_price_input("price-label").into_string()))
}

pub async fn get_beds() -> Result<Html<String>, AppError> {
    let choices = vec!["Any", "1", "2", "3", "4", "5"];

    let html = html! {
        (render_beds_baths_selection_drop_down(choices, "Any"))
        (render_selection_label("Any", "bed-label"))
    }
    .into_string();

    Ok(Html(html))
}

pub async fn get_baths() -> Result<Html<String>, AppError> {
    let choices = vec!["Any", "1", "2", "3", "4", "5"];

    let html = html! {
        (render_beds_baths_selection_drop_down(choices, "Any"))
        (render_selection_label("Any", "bath-label"))
    }
    .into_string();

    Ok(Html(html))
}
