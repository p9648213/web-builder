use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::Html;
use deadpool_postgres::Pool;
use maud::html;

use crate::models::error::AppError;
use crate::models::rso_data::{
    HotPropertyParams, LocationParams, PropertyParams, PropertyTypeParams, RsoData,
    SearchResultParams,
};
use crate::views::real_estate::home::{
    render_hot_properties_slider, render_location_selection_drop_down,
    render_property_types_selection_drop_down, render_selection_label,
};
use crate::views::real_estate::property_details::render_detail;
use crate::views::real_estate::search_result;

use super::pages::{PropertyQuery, SearchQuery};

pub async fn get_locations(State(pg_pool): State<Pool>) -> Result<Html<String>, AppError> {
    let row = RsoData::get_rso_data_by_user_id(
        1,
        &pg_pool,
        vec!["filter_id_sale", "identifier_id", "api_key"],
    )
    .await?;

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
    let row = RsoData::get_rso_data_by_user_id(
        1,
        &pg_pool,
        vec!["filter_id_sale", "identifier_id", "api_key"],
    )
    .await?;

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

pub async fn get_hot_properties(State(pg_pool): State<Pool>) -> Result<Html<String>, AppError> {
    let row = RsoData::get_rso_data_by_user_id(
        1,
        &pg_pool,
        vec!["filter_id_featured", "identifier_id", "api_key"],
    )
    .await?;

    if let Some(row) = row {
        let rso_data = RsoData::try_from(row);

        let p_agency_filterid = rso_data.filter_id_featured.ok_or_else(|| {
            tracing::error!("No filter_id_featured column or value is null");
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

        let property_response = RsoData::get_rso_hot_properties(property_params).await?;

        let html = html! {
            (render_hot_properties_slider(property_response.property))
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

pub async fn get_property(
    State(pg_pool): State<Pool>,
    property_query: Query<PropertyQuery>,
) -> Result<Html<String>, AppError> {
    if property_query.0.id.is_none() || property_query.0.listing_type.is_none() {
        return Ok(Html("Property not found".to_string()));
    }

    let row = RsoData::get_rso_data_by_user_id(
        1,
        &pg_pool,
        vec![
            "identifier_id",
            "api_key",
            "filter_id_sale",
            "filter_id_long",
            "filter_id_short",
        ],
    )
    .await?;

    if let Some(row) = row {
        let rso_data = RsoData::try_from(row);

        let p1 = rso_data.identifier_id.ok_or_else(|| {
            tracing::error!("No identifier_id column or value is null");
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
        })?;

        let p2 = rso_data.api_key.ok_or_else(|| {
            tracing::error!("No api_key column or value is null");
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
        })?;

        let property_id = property_query.0.id.unwrap();
        let listing_type = property_query.0.listing_type.unwrap();

        let p_agency_filterid = match listing_type.as_str() {
            "sale" | "new-dev" => rso_data.filter_id_sale.ok_or_else(|| {
                tracing::error!("No filter_id_sale column or value is null");
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
            })?,
            "short-rental" => rso_data.filter_id_short.ok_or_else(|| {
                tracing::error!("No filter_id_short column or value is null");
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
            })?,
            "long-rental" => rso_data.filter_id_long.ok_or_else(|| {
                tracing::error!("No filter_id_long column or value is null");
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
            })?,
            _ => {
                tracing::error!("Invalid listing type in get_search_result");
                Err(AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Server Error",
                ))
            }?,
        };

        let property_params = PropertyParams {
            p_agency_filterid: p_agency_filterid.to_string(),
            p1: p1.to_string(),
            p2: p2.to_string(),
            p_lang: "1".to_string(),
            p_ref_id: property_id.to_string(),
            benchmark: "Public_AR_Current".to_string(),
            p_sandbox: "false".to_string(),
            p_ignore_hash: "true".to_string(),
            p_shownewdevname: "true".to_string(),
            p_dimension: "1".to_string(),
            p_virtual_tours: "2".to_string(),
            p_currency: "EUR".to_string(),
            subversion: "v6.1".to_string(),
        };

        let property_response = RsoData::get_rso_property(property_params).await?;

        let html = html! {
            (render_detail(&property_response.property))
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
    let row = RsoData::get_rso_data_by_user_id(
        1,
        &pg_pool,
        vec![
            "identifier_id",
            "api_key",
            "filter_id_sale",
            "filter_id_long",
            "filter_id_short",
        ],
    )
    .await?;

    if let Some(row) = row {
        let rso_data = RsoData::try_from(row);

        let p1 = rso_data.identifier_id.ok_or_else(|| {
            tracing::error!("No identifier_id column or value is null");
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
        })?;

        let p2 = rso_data.api_key.ok_or_else(|| {
            tracing::error!("No api_key column or value is null");
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
        })?;

        let page_no = search_query.0.page.unwrap_or(1);

        let listing_type = search_query.0.listing_type.unwrap_or("sale".to_string());

        let p_agency_filterid = match listing_type.as_str() {
            "sale" | "new-dev" => rso_data.filter_id_sale.ok_or_else(|| {
                tracing::error!("No filter_id_sale column or value is null");
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
            })?,
            "short-rental" => rso_data.filter_id_short.ok_or_else(|| {
                tracing::error!("No filter_id_short column or value is null");
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
            })?,
            "long-rental" => rso_data.filter_id_long.ok_or_else(|| {
                tracing::error!("No filter_id_long column or value is null");
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
            })?,
            _ => {
                tracing::error!("Invalid listing type in get_search_result");
                Err(AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Server Error",
                ))
            }?,
        };

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

        let search_response = RsoData::get_rso_search_result(search_result_params).await?;

        let search_theme = search_query.0.theme.unwrap_or(1);

        let html = html! {
            @match search_theme {
                1 => {
                    (search_result::render_property_grids_1(&search_response.property, search_response.query_info.property_count, search_response.query_info.properties_per_page, search_response.query_info.current_page, &listing_type))
                }
                2 => {
                    (search_result::render_property_grids_2())
                }
                3 => {
                    (search_result::render_property_grids_3())
                }
                4 => {
                    (search_result::render_property_grids_4(&search_response.property, search_response.query_info.property_count, search_response.query_info.properties_per_page, search_response.query_info.current_page, &listing_type))
                }
                _ => {}
            }
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
