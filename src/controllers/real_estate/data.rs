use axum::response::Html;
use maud::html;

use crate::{models::error::AppError, views::real_estate::home};

pub async fn get_listing_type() -> Result<Html<String>, AppError> {
    let listing_types = vec!["Resales", "New Development", "Short rental", "Long rental"];

    let html = html! {
        (home::render_selection_drop_down_1(listing_types, "Resales"))
        (home::render_selection_label_1("Resales", "listing-type-label"))
    }
    .into_string();

    Ok(Html(html))
}

pub async fn get_price() -> Result<Html<String>, AppError> {
    Ok(Html(
        home::render_price_input_1("price-label").into_string(),
    ))
}

pub async fn get_beds() -> Result<Html<String>, AppError> {
    let choices = vec!["Any", "1", "2", "3", "4", "5"];

    let html = html! {
        (home::render_beds_baths_selection_drop_down_1(choices, "Any"))
        (home::render_selection_label_1("Any", "bed-label"))
    }
    .into_string();

    Ok(Html(html))
}

pub async fn get_baths() -> Result<Html<String>, AppError> {
    let choices = vec!["Any", "1", "2", "3", "4", "5"];

    let html = html! {
        (home::render_beds_baths_selection_drop_down_1(choices, "Any"))
        (home::render_selection_label_1("Any", "bath-label"))
    }
    .into_string();

    Ok(Html(html))
}
