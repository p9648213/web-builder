use deadpool_postgres::Pool;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

use crate::utilities::db::{excute, query_optional};

use super::error::AppError;

const RSO_URL: &str = "http://release-238.git.env1.resales-online.com/WebApi/V6-0";

//........................................
//.RRRRRRRRR......SSSSSS.......OOOOOO.....
//.RRRRRRRRRRR..SSSSSSSSS....OOOOOOOOOO...
//.RRRRRRRRRRR..SSSSSSSSSS..OOOOOOOOOOOO..
//.RRR.....RRR..SSS...SSSS..OOOO....OOOO..
//.RRR.....RRR..SSSS........OOO......OOO..
//.RRRRRRRRRRR..SSSSSSS....SOOO......OOO..
//.RRRRRRRRRR....SSSSSSSS..SOOO......OOO..
//.RRRRRRRR........SSSSSSS.SOOO......OOO..
//.RRR..RRRR...........SSSS.OOO......OOO..
//.RRR...RRRR..RSSS....SSSS.OOOO....OOOO..
//.RRR....RRRR..SSSSSSSSSS..OOOOOOOOOOOO..
//.RRR....RRRR..SSSSSSSSSS...OOOOOOOOOO...
//.RRR.....RRRR...SSSSSS.......OOOOOO.....
//........................................

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

        let url = reqwest::Url::parse_with_params(
            format!("{}/SearchLocations.php", RSO_URL).as_str(),
            params,
        )
        .map_err(|err| {
            tracing::error!("Error parse rso location params: {}", err.to_string());
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        let response = reqwest::get(url).await.map_err(|err| {
            tracing::error!("Error getting rso location: {}", err.to_string());
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        let text = response.text().await.map_err(|err| {
            tracing::error!("Error parsing rso location text: {}", err.to_string());
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        let location: LocationResponse = serde_json::from_str(&text).map_err(|err| {
            tracing::error!("Failed to deserialize rso location: {}", err.to_string());
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        Ok(location)
    }

    pub async fn get_rso_property_type(
        params: PropertyTypeParams,
    ) -> Result<PropertyTypeResponse, AppError> {
        let params = [
            ("p_agency_filterid", params.p_agency_filterid),
            ("p1", params.p1),
            ("p2", params.p2),
            ("P_sandbox", params.p_sandbox),
            ("benchmark", params.benchmark),
            ("P_SortType", params.p_sort_type),
            ("P_All", params.p_all),
            ("P_IgnoreHash", params.p_ignore_hash),
            ("P_Lange", params.p_lang),
        ];

        let url = reqwest::Url::parse_with_params(
            format!("{}/SearchPropertyTypes.php", RSO_URL).as_str(),
            params,
        )
        .map_err(|err| {
            tracing::error!("Error parse rso property type params: {}", err.to_string());
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        let response = reqwest::get(url).await.map_err(|err| {
            tracing::error!("Error getting rso property type: {}", err.to_string());
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        let text = response.text().await.map_err(|err| {
            tracing::error!("Error parsing rso property type text: {}", err.to_string());
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        let property_type: PropertyTypeResponse = serde_json::from_str(&text).map_err(|err| {
            tracing::error!(
                "Failed to deserialize rso property type: {}",
                err.to_string()
            );
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        Ok(property_type)
    }
}

//..............................................................................................
//.LLL...........OOOOOO........CCCCCC.......AAAA....AATTTTTTTTTTTII....OOOOOOO....OONN....NNNN..
//.LLL.........OOOOOOOOOO....CCCCCCCCC......AAAAA...AATTTTTTTTTTTII...OOOOOOOOO...OONNN...NNNN..
//.LLL........OOOOOOOOOOOO..CCCCCCCCCCC.....AAAAA...AATTTTTTTTTTTII..IOOOOOOOOOO..OONNN...NNNN..
//.LLL........OOOO....OOOO..CCCC...CCCC....AAAAAA.......TTTT...TTII.IIOOO...OOOOO.OONNNN..NNNN..
//.LLL........OOO......OOO..CCC.....CC.....AAAAAAA......TTTT...TTII.IIOO.....OOOO.OONNNNN.NNNN..
//.LLL.......LOOO......OOOOOCCC...........AAAA.AAA......TTTT...TTII.IIO.......OOO.OONNNNN.NNNN..
//.LLL.......LOOO......OOOOOCCC...........AAA..AAAA.....TTTT...TTII.IIO.......OOO.OONNNNNNNNNN..
//.LLL.......LOOO......OOOOOCCC...........AAAAAAAAA.....TTTT...TTII.IIO.......OOO.OONN.NNNNNNN..
//.LLL........OOO......OOO..CCC.....CC...AAAAAAAAAA.....TTTT...TTII.IIOO.....OOOO.OONN.NNNNNNN..
//.LLL........OOOO....OOOO..CCCC...CCCC..AAAAAAAAAAA....TTTT...TTII.IIOOO...OOOOO.OONN..NNNNNN..
//.LLLLLLLLLL.OOOOOOOOOOOO..CCCCCCCCCCC..AAA.....AAA....TTTT...TTII..IOOOOOOOOOO..OONN..NNNNNN..
//.LLLLLLLLLL..OOOOOOOOOO....CCCCCCCCC..CAAA.....AAAA...TTTT...TTII...OOOOOOOOO...OONN...NNNNN..
//.LLLLLLLLLL....OOOOOO........CCCCCC...CAA......AAAA...TTTT...TTII....OOOOOOO....OONN....NNNN..
//..............................................................................................

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
    pub version: u32,
    pub service: String,
    pub datetime: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationQueryInfo {
    #[serde(rename = "ApiId")]
    pub api_id: u32,
    #[serde(rename = "LocationCount")]
    pub location_count: u32,
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
    pub location: LocationDynamic,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LocationDynamic {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProvinceAreaDynamic {
    Single(ProvinceArea),
    Multiple(Vec<ProvinceArea>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationData {
    #[serde(rename = "Country")]
    pub country: String,
    #[serde(rename = "ProvinceArea")]
    pub province_area: ProvinceAreaDynamic,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationResponse {
    pub transaction: Transaction,
    #[serde(rename = "QueryInfo")]
    pub query_info: LocationQueryInfo,
    #[serde(rename = "LocationData")]
    pub location_data: LocationData,
}

//.........................................................................
//.PPPPPPPPP...RRRRRRRRR....TTTTTTTTTTTYYY....YYYY.PPPPPPPPP...EEEEEEEEEE..
//.PPPPPPPPPP..RRRRRRRRRRR..TTTTTTTTTTTYYY....YYY..PPPPPPPPPP..EEEEEEEEEE..
//.PPPPPPPPPP..RRRRRRRRRRR..TTTTTTTTTTTYYYY..YYYY..PPPPPPPPPP..EEEEEEEEEE..
//.PPP....PPPP.RRR.....RRR......TTT.....YYY..YYY...PPP....PPPP.EEE.........
//.PPP....PPPP.RRR.....RRR......TTT.....YYYYYYYY...PPP....PPPP.EEE.........
//.PPPPPPPPPP..RRRRRRRRRRR......TTT......YYYYYY....PPPPPPPPPP..EEEEEEEEEE..
//.PPPPPPPPPP..RRRRRRRRRR.......TTT.......YYYY.....PPPPPPPPPP..EEEEEEEEEE..
//.PPPPPPPPP...RRRRRRRR.........TTT.......YYYY.....PPPPPPPPP...EEEEEEEEEE..
//.PPP.........RRR..RRRR........TTT.......YYYY.....PPP.........EEE.........
//.PPP.........RRR...RRRR.......TTT.......YYYY.....PPP.........EEE.........
//.PPP.........RRR....RRRR......TTT.......YYYY.....PPP.........EEEEEEEEEE..
//.PPP.........RRR....RRRR......TTT.......YYYY.....PPP.........EEEEEEEEEE..
//.PPP.........RRR.....RRRR.....TTT.......YYYY.....PPP.........EEEEEEEEEE..
//.........................................................................
#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyTypeParams {
    pub p_agency_filterid: String,
    pub p1: String,
    pub p2: String,
    pub p_sandbox: String,
    pub benchmark: String,
    pub p_sort_type: String,
    pub p_all: String,
    pub p_ignore_hash: String,
    pub p_lang: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyTypeResponse {
    pub transaction: Transaction,
    #[serde(rename = "QueryInfo")]
    pub query_info: PropertyTypeQueryInfo,
    #[serde(rename = "PropertyTypes")]
    pub property_types: PropertyTypes,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyTypeQueryInfo {
    #[serde(rename = "ApiId")]
    pub api_id: u32,
    #[serde(rename = "PropertyTypesCount")]
    pub property_types_count: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyTypes {
    #[serde(rename = "PropertyType")]
    pub property_type: Vec<PropertyType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyType {
    #[serde(rename = "Type")]
    pub prop_type: String,
    #[serde(rename = "OptionValue")]
    pub option_value: String,
    #[serde(rename = "SubType")]
    pub sub_types: Vec<PropertySubType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertySubType {
    #[serde(rename = "Type")]
    pub prop_sub_type: String,
    #[serde(rename = "OptionValue")]
    pub sub_type_option_value: String,
}
