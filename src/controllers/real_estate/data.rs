use axum::{
    extract::{Query, Request},
    response::Html,
};
use maud::html;
use reqwest::StatusCode;
use serde::Deserialize;

use crate::{models::error::AppError, views::real_estate::home};

#[derive(Deserialize, Debug)]
pub struct ListingTypeQuery {
    #[serde(rename = "type")]
    pub listing_type: Option<String>,
}

pub async fn get_listing_type(request: Request) -> Result<Html<String>, AppError> {
    let uri = request.uri();

    let listing_type_query: Query<ListingTypeQuery> =
        Query::try_from_uri(uri).map_err(|error| {
            tracing::error!("Failed to extract search query: {}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

    let listing_type = match listing_type_query
        .0
        .listing_type
        .unwrap_or("resales".to_owned())
        .as_str()
    {
        "resales" => "Resales",
        "new-development" => "New Development",
        "short-rental" => "Short Rental",
        "long-rental" => "Long Rental",
        _ => {
            tracing::error!("Invalid listing type");
            Err(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Server Error",
            ))
        }?,
    };

    let listing_types = vec!["Resales", "New Development", "Short rental", "Long rental"];

    let html = html! {
        (home::render_listing_type_selection_drop_down_1(listing_types, listing_type, "listing-type-selection"))
        (home::render_selection_label_1(listing_type, "listing-type-label"))
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
