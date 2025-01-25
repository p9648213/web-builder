use axum::{
    extract::{Query, Request, State},
    http::HeaderValue,
    response::Html,
};
use deadpool_postgres::Pool;
use reqwest::StatusCode;
use serde::Deserialize;

use crate::{
    models::{error::AppError, website_setting_website::WebsiteJoinWebsiteSetting},
    views::real_estate::pages::{
        render_contact_page, render_home_page, render_property_details_page,
        render_search_result_page,
    },
};

#[derive(Deserialize, Debug)]
pub struct SearchQuery {
    pub page: Option<u32>,
    pub listing_type: Option<String>,
    pub theme: Option<i32>,
}

#[derive(Deserialize, Debug)]
pub struct PropertyQuery {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub listing_type: Option<String>,
    pub theme: Option<i32>,
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

    let row = WebsiteJoinWebsiteSetting::get_website_setting_by_domain(
        host,
        &pg_pool,
        None,
        Some(vec!["header_theme", "footer_theme"]),
        Some("w"),
        Some("s"),
    )
    .await?;

    if let Some(row) = row {
        let website_setting_website = WebsiteJoinWebsiteSetting::try_from(&row, "w_", "s_");

        let header_theme = website_setting_website
            .website_setting
            .header_theme
            .ok_or_else(|| {
                tracing::error!("No header_theme column or value is null");
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
            })?;

        let footer_theme = website_setting_website
            .website_setting
            .footer_theme
            .ok_or_else(|| {
                tracing::error!("No footer_theme column or value is null");
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
            })?;

        Ok(Html(
            render_home_page(header_theme, footer_theme).into_string(),
        ))
    } else {
        Err(AppError::new(StatusCode::NOT_FOUND, "Domain not found"))
    }
}

pub async fn get_real_estate_search_result_page(
    search_query: Query<SearchQuery>,
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

    let row = WebsiteJoinWebsiteSetting::get_website_setting_by_domain(
        host,
        &pg_pool,
        None,
        Some(vec!["header_theme", "footer_theme", "search_theme"]),
        Some("w"),
        Some("s"),
    )
    .await?;

    if let Some(row) = row {
        let website_setting_website = WebsiteJoinWebsiteSetting::try_from(&row, "w_", "s_");

        let header_theme = website_setting_website
            .website_setting
            .header_theme
            .ok_or_else(|| {
                tracing::error!("No header_theme column or value is null");
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
            })?;

        let footer_theme = website_setting_website
            .website_setting
            .footer_theme
            .ok_or_else(|| {
                tracing::error!("No footer_theme column or value is null");
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
            })?;

        let search_theme = website_setting_website
            .website_setting
            .search_theme
            .ok_or_else(|| {
                tracing::error!("No search_theme column or value is null");
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
            })?;

        Ok(Html(
            render_search_result_page(search_query.0, header_theme, footer_theme, search_theme)
                .into_string(),
        ))
    } else {
        Err(AppError::new(StatusCode::NOT_FOUND, "Domain not found"))
    }
}

pub async fn get_real_estate_property_page(
    property_query: Query<PropertyQuery>,
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

    let row = WebsiteJoinWebsiteSetting::get_website_setting_by_domain(
        host,
        &pg_pool,
        None,
        Some(vec!["header_theme", "footer_theme", "property_theme"]),
        Some("w"),
        Some("s"),
    )
    .await?;

    if let Some(row) = row {
        let website_setting_website = WebsiteJoinWebsiteSetting::try_from(&row, "w_", "s_");

        let header_theme = website_setting_website
            .website_setting
            .header_theme
            .ok_or_else(|| {
                tracing::error!("No header_theme column or value is null");
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
            })?;

        let footer_theme = website_setting_website
            .website_setting
            .footer_theme
            .ok_or_else(|| {
                tracing::error!("No footer_theme column or value is null");
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
            })?;

        let property_theme = website_setting_website
            .website_setting
            .property_theme
            .ok_or_else(|| {
                tracing::error!("No property_theme column or value is null");
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
            })?;

        Ok(Html(
            render_property_details_page(
                property_query.0,
                header_theme,
                footer_theme,
                property_theme,
            )
            .into_string(),
        ))
    } else {
        Err(AppError::new(StatusCode::NOT_FOUND, "Domain not found"))
    }
}

pub async fn get_real_estate_contact_page(
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

    let row = WebsiteJoinWebsiteSetting::get_website_setting_by_domain(
        host,
        &pg_pool,
        None,
        Some(vec!["header_theme", "footer_theme", "contact_theme"]),
        Some("w"),
        Some("s"),
    )
    .await?;

    if let Some(row) = row {
        let website_setting_website = WebsiteJoinWebsiteSetting::try_from(&row, "w_", "s_");

        let header_theme = website_setting_website
            .website_setting
            .header_theme
            .ok_or_else(|| {
                tracing::error!("No header_theme column or value is null");
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
            })?;

        let footer_theme = website_setting_website
            .website_setting
            .footer_theme
            .ok_or_else(|| {
                tracing::error!("No footer_theme column or value is null");
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
            })?;

        let contact_theme = website_setting_website
            .website_setting
            .contact_theme
            .ok_or_else(|| {
                tracing::error!("No contact_theme column or value is null");
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
            })?;

        Ok(Html(
            render_contact_page(header_theme, footer_theme, contact_theme).into_string(),
        ))
    } else {
        Err(AppError::new(StatusCode::NOT_FOUND, "Domain not found"))
    }
}
