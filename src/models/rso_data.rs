use deadpool_postgres::Pool;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

use crate::utilities::db::{excute, query_optional};

use super::error::AppError;

const RSO_URL: &str = "https://webapi.resales-online.com/V6/SearchLocations.php";

pub struct RsoData {
    pub id: Option<i32>,
    pub user_id: Option<i32>,
    pub identifier_id: Option<i32>,
    pub api_key: Option<String>,
    pub filter_id_sale: Option<i32>,
    pub filter_id_long: Option<i32>,
    pub filter_id_short: Option<i32>,
    pub filter_id_featured: Option<i32>,
}

impl RsoData {
    pub fn new(
        id: Option<i32>,
        user_id: Option<i32>,
        identifier_id: Option<i32>,
        api_key: Option<String>,
        filter_id_sale: Option<i32>,
        filter_id_long: Option<i32>,
        filter_id_short: Option<i32>,
        filter_id_featured: Option<i32>,
    ) -> Self {
        Self {
            id,
            user_id,
            identifier_id,
            api_key,
            filter_id_sale,
            filter_id_long,
            filter_id_short,
            filter_id_featured,
        }
    }

    pub fn try_from(row: Row) -> Self {
        let id: Option<i32> = row.try_get("id").unwrap_or(None);
        let user_id: Option<i32> = row.try_get("user_id").unwrap_or(None);
        let identifier_id: Option<i32> = row.try_get("identifier_id").unwrap_or(None);
        let api_key: Option<String> = row.try_get("api_key").unwrap_or(None);
        let filter_id_sale: Option<i32> = row.try_get("filter_id_sale").unwrap_or(None);
        let filter_id_long: Option<i32> = row.try_get("filter_id_long").unwrap_or(None);
        let filter_id_short: Option<i32> = row.try_get("filter_id_short").unwrap_or(None);
        let filter_id_featured: Option<i32> = row.try_get("filter_id_featured").unwrap_or(None);

        Self {
            id,
            user_id,
            identifier_id,
            api_key,
            filter_id_sale,
            filter_id_long,
            filter_id_short,
            filter_id_featured,
        }
    }

    pub async fn create_rso_data_by_user_id(
        user_id: i32,
        rso_data: &RsoData,
        pool: &Pool,
    ) -> Result<u64, AppError> {
        excute(
            "INSERT INTO rso_data (user_id, identifier_id, api_key, filter_id_sale, filter_id_long, filter_id_short, filter_id_featured) VALUES ($1, $2, $3, $4, $5, $6, $7)",
            &[
                &user_id,
                &rso_data.identifier_id,
                &rso_data.api_key,
                &rso_data.filter_id_sale,
                &rso_data.filter_id_long,
                &rso_data.filter_id_short,
                &rso_data.filter_id_featured,
            ],
            pool,
        )
        .await
    }

    pub async fn update_rso_data_by_user_id(
        user_id: i32,
        rso_data: &RsoData,
        pool: &Pool,
    ) -> Result<u64, AppError> {
        excute(
            "UPDATE rso_data SET identifier_id = $1, api_key = $2, filter_id_sale = $3, filter_id_long = $4, filter_id_short = $5, filter_id_featured = $6 WHERE user_id = $7",
            &[
                &rso_data.identifier_id,
                &rso_data.api_key,
                &rso_data.filter_id_sale,
                &rso_data.filter_id_long,
                &rso_data.filter_id_short,
                &rso_data.filter_id_featured,
                &user_id,
            ],
            pool,
        )
        .await
    }

    pub async fn update_status_rso_data_by_user_id(
        user_id: i32,
        status: bool,
        pool: &Pool,
    ) -> Result<u64, AppError> {
        excute(
            "UPDATE rso_data SET status = $1 WHERE user_id = $2",
            &[&status, &user_id],
            pool,
        )
        .await
    }

    pub async fn get_rso_data_by_user_id(
        user_id: i32,
        pool: &Pool,
    ) -> Result<Option<Row>, AppError> {
        query_optional(
            "SELECT * FROM rso_data WHERE user_id = $1",
            &[&user_id],
            pool,
        )
        .await
    }

    pub async fn get_rso_location(params: LocationParams) -> Result<LocationResponse, AppError> {
        let params = [
            ("p_agency_filterid", params.p_agency_filterid),
            ("p1", params.p1),
            ("p2", params.p2),
            ("P_sandbox", params.p_sandbox),
            ("benchmark", params.benchmark),
            ("P_SortType", params.p_sort_type),
            ("P_All", params.p_all),
            ("P_IgnoreHash", params.p_ignore_hash),
        ];

        let url = reqwest::Url::parse_with_params(RSO_URL, params).map_err(|err| {
            tracing::error!("Error parse rso location params: {}", err.to_string());
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        let response = reqwest::get(url).await.map_err(|err| {
            tracing::error!("Error getting rso location: {}", err.to_string());
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        let text = response.text().await.map_err(|err| {
            tracing::error!("Error getting rso location: {}", err.to_string());
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        let location: LocationResponse = serde_json::from_str(&text).map_err(|err| {
            tracing::error!("Failed to deserialize rso location: {}", err.to_string());
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        Ok(location)
    }
}

pub struct LocationParams {
    pub p_agency_filterid: String,
    pub p1: String,
    pub p2: String,
    pub p_sandbox: String,
    pub benchmark: String,
    pub p_sort_type: String,
    pub p_all: String,
    pub p_ignore_hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub status: String,
    pub errordescription: String,
    #[serde(rename = "incomingIp")]
    pub incoming_ip: String,
    pub version: u8,
    pub service: String,
    pub datetime: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryInfo {
    #[serde(rename = "ApiId")]
    pub api_id: u32,
    #[serde(rename = "LocationCount")]
    pub location_count: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProvinceArea {
    #[serde(rename = "ProvinceAreaName")]
    pub province_area_name: String,
    #[serde(rename = "Locations")]
    pub locations: Locations,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Locations {
    #[serde(rename = "Location")]
    pub location: Location,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Location {
    Single(String),
    Multiple(Vec<String>),
}

impl Location {
    pub fn as_vec(&self) -> Vec<String> {
        match self {
            Location::Single(s) => vec![s.clone()],
            Location::Multiple(vec) => vec.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationData {
    #[serde(rename = "Country")]
    pub country: String,
    #[serde(rename = "ProvinceArea")]
    pub province_area: ProvinceArea,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationResponse {
    pub transaction: Transaction,
    #[serde(rename = "QueryInfo")]
    pub query_info: QueryInfo,
    #[serde(rename = "LocationData")]
    pub location_data: LocationData,
}
