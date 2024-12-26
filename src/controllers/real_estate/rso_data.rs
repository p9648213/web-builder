use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::Html;
use deadpool_postgres::Pool;
use maud::html;
use serde::Deserialize;

use crate::models::error::AppError;
use crate::models::rso_data::{
    HotPropertyParams, LocationParams, PropertyTypeParams, RsoData, SearchResultParams,
};
use crate::views::real_estate::components::{
    render_hot_property_slider, render_location_selection_drop_down,
    render_property_types_selection_drop_down, render_selection_label,
};
use crate::views::real_estate::search_components;

#[derive(Deserialize, Debug)]
pub struct SearchQuery {
    pub page: Option<u32>,
}

pub async fn get_locations(State(pg_pool): State<Pool>) -> Result<Html<String>, AppError> {
    let row = RsoData::get_rso_data_by_user_id(1, &pg_pool).await?;

    if let Some(row) = row {
        let rso_data = RsoData::try_from(row);

        let p_agency_filterid = rso_data.filter_id_sale.ok_or_else(|| {
            tracing::error!("No filter_id_sale column or value is null");
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
        })?;

        let p1 = rso_data.identifier_id.ok_or_else(|| {
            tracing::error!("No identifier_id column or value is null");
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
        })?;

        let p2 = rso_data.api_key.ok_or_else(|| {
            tracing::error!("No api_key column or value is null");
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
        })?;

        let location_params = LocationParams {
            p_agency_filterid: p_agency_filterid.to_string(),
            p1: p1.to_string(),
            p2: p2.to_string(),
            p_sandbox: "true".to_string(),
            benchmark: "Public_AR_Current".to_string(),
            p_sort_type: "0".to_string(),
            p_all: "false".to_string(),
            p_ignore_hash: "true".to_string(),
        };

        let location = RsoData::get_rso_location(location_params).await?;

        let html = html! {
            (render_location_selection_drop_down(location.location_data.province_area, "All"))
            (render_selection_label("All", "location-label"))
        }
        .into_string();

        Ok(Html(html))
    } else {
        tracing::error!("No rso data found for user id 1");
        return Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Server Error",
        ));
    }
}

pub async fn get_property_types(State(pg_pool): State<Pool>) -> Result<Html<String>, AppError> {
    let row = RsoData::get_rso_data_by_user_id(1, &pg_pool).await?;

    if let Some(row) = row {
        let rso_data = RsoData::try_from(row);

        let p_agency_filterid = rso_data.filter_id_sale.ok_or_else(|| {
            tracing::error!("No filter_id_sale column or value is null");
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
        })?;

        let p1 = rso_data.identifier_id.ok_or_else(|| {
            tracing::error!("No identifier_id column or value is null");
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
        })?;

        let p2 = rso_data.api_key.ok_or_else(|| {
            tracing::error!("No api_key column or value is null");
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
        })?;

        let property_type_params = PropertyTypeParams {
            p_agency_filterid: p_agency_filterid.to_string(),
            p1: p1.to_string(),
            p2: p2.to_string(),
            p_sandbox: "true".to_string(),
            benchmark: "Public_AR_Current".to_string(),
            p_sort_type: "0".to_string(),
            p_all: "false".to_string(),
            p_ignore_hash: "true".to_string(),
            p_lang: "1".to_string(),
        };

        let property_type = RsoData::get_rso_property_type(property_type_params).await?;

        let html = html! {
            (render_property_types_selection_drop_down(property_type.property_types.property_type))
            (render_selection_label("All", "property-types-label"))
        }
        .into_string();

        Ok(Html(html))
    } else {
        tracing::error!("No rso data found for user id 1");
        return Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Server Error",
        ));
    }
}

pub async fn get_property_types_slider(
    State(pg_pool): State<Pool>,
) -> Result<Html<String>, AppError> {
    let row = RsoData::get_rso_data_by_user_id(1, &pg_pool).await?;

    if let Some(row) = row {
        let rso_data = RsoData::try_from(row);

        let p_agency_filterid = rso_data.filter_id_featured.ok_or_else(|| {
            tracing::error!("No filter_id_sale column or value is null");
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
        })?;

        let p1 = rso_data.identifier_id.ok_or_else(|| {
            tracing::error!("No identifier_id column or value is null");
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
        })?;

        let p2 = rso_data.api_key.ok_or_else(|| {
            tracing::error!("No api_key column or value is null");
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
        })?;

        let property_params = HotPropertyParams {
            p_agency_filterid: p_agency_filterid.to_string(),
            p1: p1.to_string(),
            p2: p2.to_string(),
            p_lang: "1".to_string(),
            p_sort_type: "5".to_string(),
            benchmark: "Public_AR_Current".to_string(),
            p_sandbox: "false".to_string(),
            p_currency: "EUR".to_string(),
            p_images: "5".to_string(),
            p_ignore_hash: "true".to_string(),
            p_shownewdevname: "true".to_string(),
            p_include_rented: "1".to_string(),
        };

        let property_response = RsoData::get_rso_property(property_params).await?;

        let html = html! {
            (render_hot_property_slider(property_response.property))
        }
        .into_string();

        Ok(Html(html))
    } else {
        tracing::error!("No rso data found for user id 1");
        return Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Server Error",
        ));
    }
}

pub async fn get_search_result(
    State(pg_pool): State<Pool>,
    search_query: Query<SearchQuery>,
) -> Result<Html<String>, AppError> {
    let row = RsoData::get_rso_data_by_user_id(1, &pg_pool).await?;

    if let Some(row) = row {
        let rso_data = RsoData::try_from(row);

        let p_agency_filterid = rso_data.filter_id_featured.ok_or_else(|| {
            tracing::error!("No filter_id_sale column or value is null");
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
        })?;

        let p1 = rso_data.identifier_id.ok_or_else(|| {
            tracing::error!("No identifier_id column or value is null");
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
        })?;

        let p2 = rso_data.api_key.ok_or_else(|| {
            tracing::error!("No api_key column or value is null");
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
        })?;

        let page_no = search_query.page.unwrap_or(1);

        let search_result_params = SearchResultParams {
            p_agency_filterid: p_agency_filterid.to_string(),
            p1: p1.to_string(),
            p2: p2.to_string(),
            p_lang: "1".to_string(),
            p_sort_type: "5".to_string(),
            benchmark: "Public_AR_Current".to_string(),
            p_sandbox: "false".to_string(),
            p_currency: "EUR".to_string(),
            p_images: "5".to_string(),
            p_ignore_hash: "true".to_string(),
            p_shownewdevname: "true".to_string(),
            p_include_rented: "1".to_string(),
            p_all: "false".to_string(),
            p_dimension: "1".to_string(),
            p_must_have_features: "-1".to_string(),
            p_page_no: page_no.to_string(),
            p_page_size: "24".to_string(),
            p_virtual_tours: "2".to_string(),
        };

        let search_response = RsoData::get_search_result(search_result_params).await?;

        let html = html! {
            (search_components::render_property_grids(&search_response.property, search_response.query_info.property_count, search_response.query_info.properties_per_page, search_response.query_info.current_page))
        }
        .into_string();

        Ok(Html(html))
    } else {
        tracing::error!("No rso data found for user id 1");
        return Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Server Error",
        ));
    }
}
