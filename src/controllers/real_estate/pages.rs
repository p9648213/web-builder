use axum::{
    extract::{Query, Request, State},
    http::HeaderValue,
    response::Html,
};
use deadpool_postgres::Pool;
use reqwest::StatusCode;
use serde::Deserialize;

use crate::{
    models::{error::AppError, website::Website},
    views::real_estate::pages::{
        render_home_page, render_property_details_page, render_search_result_page,
    },
};

#[derive(Deserialize, Debug)]
pub struct SearchQuery {
    pub page: Option<u32>,
    pub listing_type: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct PropertyQuery {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub listing_type: Option<String>,
}

pub async fn get_real_estate_home_page(
    State(pg_pool): State<Pool>,
    request: Request,
) -> Result<Html<String>, AppError> {
    let default_host = HeaderValue::from_static("");

    let host = request
        .headers()
        .get("host")
        .unwrap_or(&default_host)
        .to_str()
        .map_err(|error| {
            tracing::error!("Failed to convert host header to string: {}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

    let row = Website::get_website_by_domain_name(&host, &pg_pool, vec!["id"]).await?;

    if let Some(_) = row {
        Ok(Html(render_home_page().into_string()))
    } else {
        Err(AppError::new(StatusCode::NOT_FOUND, "Domain not found"))
    }
}

pub async fn get_real_estate_search_result_page(search_query: Query<SearchQuery>) -> Html<String> {
    Html(render_search_result_page(search_query.0).into_string())
}

pub async fn get_real_estate_property_page(property_query: Query<PropertyQuery>) -> Html<String> {
    Html(render_property_details_page(property_query.0).into_string())
}
