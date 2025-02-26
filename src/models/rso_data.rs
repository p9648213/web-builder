use deadpool_postgres::Pool;
use reqwest::StatusCode;
use serde::{
    de::{self, Visitor},
    Deserialize, Serialize,
};
use tokio_postgres::Row;

use crate::utilities::db::{excute, query_optional};

use super::error::AppError;

const RSO_URL: &str = "https://webapi.resales-online.com/V6";

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

    pub fn try_from(row: &Row, prefix: Option<&str>) -> Self {
        let prefix = prefix.unwrap_or("");

        let id: Option<i32> = row
            .try_get(format!("{}id", prefix).as_str())
            .unwrap_or(None);
        let user_id: Option<i32> = row
            .try_get(format!("{}user_id", prefix).as_str())
            .unwrap_or(None);
        let identifier_id: Option<i32> = row
            .try_get(format!("{}identifier_id", prefix).as_str())
            .unwrap_or(None);
        let api_key: Option<String> = row
            .try_get(format!("{}api_key", prefix).as_str())
            .unwrap_or(None);
        let filter_id_sale: Option<i32> = row
            .try_get(format!("{}filter_id_sale", prefix).as_str())
            .unwrap_or(None);
        let filter_id_long: Option<i32> = row
            .try_get(format!("{}filter_id_long", prefix).as_str())
            .unwrap_or(None);
        let filter_id_short: Option<i32> = row
            .try_get(format!("{}filter_id_short", prefix).as_str())
            .unwrap_or(None);
        let filter_id_featured: Option<i32> = row
            .try_get(format!("{}filter_id_featured", prefix).as_str())
            .unwrap_or(None);

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
        columns: Vec<&str>,
    ) -> Result<Option<Row>, AppError> {
        let columns = columns.join(",");

        query_optional(
            &format!("SELECT {} FROM rso_data WHERE user_id = $1", columns),
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
            ("P_Lang", params.p_lang),
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

    pub async fn get_rso_hot_properties(
        params: HotPropertyParams,
    ) -> Result<SearchResponse, AppError> {
        let params = [
            ("p_agency_filterid", params.p_agency_filterid),
            ("p1", params.p1),
            ("p2", params.p2),
            ("P_Lang", params.p_lang),
            ("P_SortType", params.p_sort_type),
            ("benchmark", params.benchmark),
            ("P_sandbox", params.p_sandbox),
            ("P_Currency", params.p_currency),
            ("p_images", params.p_images),
            ("P_IgnoreHash", params.p_ignore_hash),
            ("P_shownewdevname", params.p_shownewdevname),
            ("P_IncludeRented", params.p_include_rented),
        ];

        let url = reqwest::Url::parse_with_params(
            format!("{}/SearchProperties.php", RSO_URL).as_str(),
            params,
        )
        .map_err(|err| {
            tracing::error!("Error parse rso hot property params: {}", err.to_string());
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        let response = reqwest::get(url).await.map_err(|err| {
            tracing::error!("Error getting rso hot property: {}", err.to_string());
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        let text = response.text().await.map_err(|err| {
            tracing::error!("Error parsing rso hot property text: {}", err.to_string());
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        let search: SearchResponse = serde_json::from_str(&text).map_err(|err| {
            tracing::error!(
                "Failed to deserialize rso hot property: {}",
                err.to_string()
            );
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        Ok(search)
    }

    pub async fn get_rso_search_result(
        params: SearchResultParams,
    ) -> Result<SearchResponse, AppError> {
        let params = [
            ("p_agency_filterid", params.p_agency_filterid),
            ("p1", params.p1),
            ("p2", params.p2),
            ("P_Lang", params.p_lang),
            ("P_SortType", params.p_sort_type),
            ("benchmark", params.benchmark),
            ("P_sandbox", params.p_sandbox),
            ("P_Currency", params.p_currency),
            ("p_images", params.p_images),
            ("P_IgnoreHash", params.p_ignore_hash),
            ("P_shownewdevname", params.p_shownewdevname),
            ("P_IncludeRented", params.p_include_rented),
            ("P_All", params.p_all),
            ("P_VirtualTours", params.p_virtual_tours),
            ("P_Dimension", params.p_dimension),
            ("P_MustHaveFeatures", params.p_must_have_features),
            ("P_PageNo", params.p_page_no),
            ("P_PageSize", params.p_page_size),
            ("P_Province", params.p_province.to_string()),
        ];

        let url = reqwest::Url::parse_with_params(
            format!("{}/SearchProperties.php", RSO_URL).as_str(),
            params,
        )
        .map_err(|err| {
            tracing::error!("Error parse rso search result params: {}", err.to_string());
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        let response = reqwest::get(url).await.map_err(|err| {
            tracing::error!("Error getting rso search result: {}", err.to_string());
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        let text = response.text().await.map_err(|err| {
            tracing::error!("Error parsing rso search result text: {}", err.to_string());
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        let search: SearchResponse = serde_json::from_str(&text).map_err(|err| {
            tracing::error!(
                "Failed to deserialize rso search result: {}",
                err.to_string()
            );
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        Ok(search)
    }

    pub async fn get_rso_property(params: PropertyParams) -> Result<PropertyResponse, AppError> {
        let params = [
            ("p_agency_filterid", params.p_agency_filterid),
            ("p1", params.p1),
            ("p2", params.p2),
            ("P_Lang", params.p_lang),
            ("benchmark", params.benchmark),
            ("P_sandbox", params.p_sandbox),
            ("P_Currency", params.p_currency),
            ("P_IgnoreHash", params.p_ignore_hash),
            ("P_shownewdevname", params.p_shownewdevname),
            ("P_VirtualTours", params.p_virtual_tours),
            ("P_Dimension", params.p_dimension),
            ("P_RefId", params.p_ref_id),
            ("subversion", params.subversion),
        ];

        let url = reqwest::Url::parse_with_params(
            format!("{}/PropertyDetails.php", RSO_URL).as_str(),
            params,
        )
        .map_err(|err| {
            tracing::error!("Error parse rso property params: {}", err.to_string());
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        let response = reqwest::get(url).await.map_err(|err| {
            tracing::error!("Error getting rso property: {}", err.to_string());
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        let text = response.text().await.map_err(|err| {
            tracing::error!("Error parsing rso property text: {}", err.to_string());
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        let property: PropertyResponse = serde_json::from_str(&text).map_err(|err| {
            tracing::error!("Failed to deserialize rso property: {}", err.to_string());
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        Ok(property)
    }
}

//..............................................................................
//.PPPPPPPPP.....AAAA......ARRRRRRRR.......AAAA.....AAMMM....MMMMM...SSSSSS.....
//.PPPPPPPPPP....AAAA......ARRRRRRRRR......AAAA.....AAMMM....MMMMM..MSSSSSSSS...
//.PPPPPPPPPP...AAAAAA.....ARRRRRRRRRR....AAAAAA....AAMMMM..MMMMMM.MMSSSSSSSS...
//.PPP....PPPP..AAAAAA.....ARR....RRRR....AAAAAA....AAMMMM..MMMMMM.MMSS...SSSS..
//.PPP....PPPP..AAAAAAA....ARR....RRRR....AAAAAAA...AAMMMM..MMMMMM.MMSS.........
//.PPPPPPPPPP..AAAAAAAA....ARRRRRRRRRR...AAAAAAAA...AAMMMMM.MMMMMM..MSSSSSS.....
//.PPPPPPPPPP..AAA..AAA....ARRRRRRRRR....AAA..AAA...AAMMMMMMMMMMMM...SSSSSSSS...
//.PPPPPPPPP..PAAAAAAAAA...ARRRRRRR.....RAAAAAAAAA..AAMMMMMMMMMMMM.....SSSSSSS..
//.PPP........PAAAAAAAAA...ARR..RRRR....RAAAAAAAAA..AAMMMMMMMMMMMM........SSSS..
//.PPP........PAAAAAAAAAA..ARR...RRRR...RAAAAAAAAAA.AAMMMMMMMMMMMM.MMS....SSSS..
//.PPP.......PPAA....AAAA..ARR...RRRR..RRAA....AAAA.AAMM.MMMM.MMMM.MMSSSSSSSSS..
//.PPP.......PPA......AAA..ARR....RRRR.RRA......AAA.AAMM.MMMM.MMMM..MSSSSSSSS...
//.PPP.......PPA......AAAA.ARR.....RRRRRRA......AAAAAAMM.MMMM.MMMM...SSSSSSS....
//..............................................................................

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
pub struct HotPropertyParams {
    pub p_agency_filterid: String,
    pub p1: String,
    pub p2: String,
    pub p_lang: String,
    pub p_sort_type: String,
    pub benchmark: String,
    pub p_sandbox: String,
    pub p_currency: String,
    pub p_images: String,
    pub p_ignore_hash: String,
    pub p_shownewdevname: String,
    pub p_include_rented: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResultParams {
    pub p_agency_filterid: String,
    pub p1: String,
    pub p2: String,
    pub p_lang: String,
    pub p_sort_type: String,
    pub benchmark: String,
    pub p_sandbox: String,
    pub p_currency: String,
    pub p_images: String,
    pub p_ignore_hash: String,
    pub p_shownewdevname: String,
    pub p_include_rented: String,
    pub p_all: String,
    pub p_virtual_tours: String,
    pub p_dimension: String,
    pub p_must_have_features: String,
    pub p_page_no: String,
    pub p_page_size: String,
    pub p_province: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyParams {
    pub p_agency_filterid: String,
    pub p1: String,
    pub p2: String,
    pub p_lang: String,
    pub p_ref_id: String,
    pub benchmark: String,
    pub p_sandbox: String,
    pub p_currency: String,
    pub p_ignore_hash: String,
    pub p_shownewdevname: String,
    pub p_virtual_tours: String,
    pub p_dimension: String,
    pub subversion: String,
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

//..............................................................................
//....SSSSSS....EEEEEEEEEE.....AAAA......RRRRRRRRR.......CCCCCC....HHH.....HHH..
//..SSSSSSSSS...EEEEEEEEEE.....AAAAA.....RRRRRRRRRRR...CCCCCCCCC...HHH.....HHH..
//..SSSSSSSSSS..EEEEEEEEEE.....AAAAA.....RRRRRRRRRRR..CCCCCCCCCCC..HHH.....HHH..
//..SSS...SSSS..EEE...........AAAAAA.....RRR.....RRR..CCCC...CCCC..HHH.....HHH..
//..SSSS........EEE...........AAAAAAA....RRR.....RRR..CCC.....CC...HHH.....HHH..
//..SSSSSSS.....EEEEEEEEEE...AAAA.AAA....RRRRRRRRRRR.CCCC..........HHHHHHHHHHH..
//...SSSSSSSS...EEEEEEEEEE...AAA..AAAA...RRRRRRRRRR..CCCC..........HHHHHHHHHHH..
//.....SSSSSSS..EEEEEEEEEE...AAAAAAAAA...RRRRRRRR....CCCC..........HHHHHHHHHHH..
//.........SSSS.EEE.........AAAAAAAAAA...RRR..RRRR....CCC.....CC...HHH.....HHH..
//.SSSS....SSSS.EEE.........AAAAAAAAAAA..RRR...RRRR...CCCC...CCCC..HHH.....HHH..
//..SSSSSSSSSS..EEEEEEEEEEE.AAA.....AAA..RRR....RRRR..CCCCCCCCCCC..HHH.....HHH..
//..SSSSSSSSSS..EEEEEEEEEEEAAAA.....AAAA.RRR....RRRR...CCCCCCCCC...HHH.....HHH..
//....SSSSSS....EEEEEEEEEEEAAA......AAAA.RRR.....RRRR....CCCCCC....HHH.....HHH..

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResponse {
    pub transaction: SearchTransaction,
    #[serde(rename = "QueryInfo")]
    pub query_info: SearchQueryInfo,
    #[serde(rename = "Property")]
    pub property: Vec<SearchProperty>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchTransaction {
    pub status: String,
    #[serde(rename = "incomingIp")]
    pub incoming_ip: String,
    pub version: String,
    pub subversion: String,
    pub service: String,
    pub datetime: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchQueryInfo {
    #[serde(rename = "ApiId")]
    pub app_id: String,
    #[serde(rename = "QueryId")]
    pub query_id: String,
    #[serde(rename = "SearchType")]
    pub search_type: String,
    #[serde(rename = "PropertyCount")]
    pub property_count: u32,
    #[serde(rename = "CurrentPage")]
    pub current_page: u32,
    #[serde(rename = "PropertiesPerPage")]
    pub properties_per_page: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum TextOrNum {
    Text(String),
    Num(u32),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchProperty {
    #[serde(rename = "Reference")]
    pub reference: String,
    #[serde(rename = "NewDevName")]
    pub newdev_name: String,
    #[serde(rename = "Country")]
    pub country: String,
    #[serde(rename = "Province")]
    pub province: String,
    #[serde(rename = "Area")]
    pub area: String,
    #[serde(rename = "Location")]
    pub location: String,
    #[serde(rename = "SubLocation")]
    pub sub_location: String,
    #[serde(rename = "PropertyType")]
    pub property_type: TypeProperty,
    #[serde(rename = "Status")]
    pub status: StatusProperty,
    #[serde(rename = "Bedrooms")]
    pub bedrooms: String,
    #[serde(rename = "Bathrooms")]
    pub bathrooms: String,
    #[serde(rename = "Currency")]
    pub currency: String,
    #[serde(rename = "Price")]
    pub price: Option<String>,
    #[serde(rename = "RentalPrice1")]
    pub rental_price_1: Option<u32>,
    #[serde(rename = "RentalPrice2")]
    pub rental_price_2: Option<u32>,
    #[serde(rename = "OriginalPrice")]
    pub original_price: Option<u32>,
    #[serde(rename = "Dimensions")]
    pub dimensions: String,
    #[serde(rename = "Built")]
    pub built: TextOrNum,
    #[serde(rename = "Terrace")]
    pub terrace: TextOrNum,
    #[serde(rename = "GardenPlot")]
    pub garden_plot: TextOrNum,
    #[serde(rename = "CO2Rated")]
    pub co2_rated: String,
    #[serde(rename = "EnergyRated")]
    pub energy_rated: String,
    #[serde(rename = "OwnProperty")]
    pub own_property: String,
    #[serde(rename = "Pool")]
    pub pool: u32,
    #[serde(rename = "Parking")]
    pub parking: u32,
    #[serde(rename = "Garden")]
    pub garden: u32,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "PropertyFeatures")]
    pub property_features: PropertyFeatures,
    #[serde(rename = "Pictures")]
    pub pictures: Option<SearchPropertyPictures>,
    #[serde(rename = "MainImage")]
    pub main_image: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct TypeProperty {
    #[serde(rename = "NameType")]
    pub name_type: String,
    #[serde(rename = "Type")]
    pub property_type: String,
    #[serde(rename = "TypeId")]
    pub type_id: String,
    pub subtypes: Vec<Subtype>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Subtype {
    pub name: String,
    pub id: String,
}

impl<'a> Deserialize<'a> for TypeProperty {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        struct PropertyTypeVisitor;

        impl<'a> Visitor<'a> for PropertyTypeVisitor {
            type Value = TypeProperty;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a map with dynamic subtype fields")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'a>,
            {
                let mut name_type = None;
                let mut property_type = None;
                let mut type_id = None;
                let mut subtypes: Vec<Subtype> = Vec::new();

                let mut subtype_names = std::collections::HashMap::new();
                let mut subtype_ids = std::collections::HashMap::new();

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "NameType" => name_type = Some(map.next_value()?),
                        "Type" => property_type = Some(map.next_value()?),
                        "TypeId" => type_id = Some(map.next_value()?),
                        key if key.starts_with("SubtypeId") => {
                            let id: String = map.next_value()?;
                            subtype_ids
                                .insert(key.strip_prefix("SubtypeId").unwrap().to_string(), id);
                        }
                        key if key.starts_with("Subtype") => {
                            let name: String = map.next_value()?;
                            subtype_names
                                .insert(key.strip_prefix("Subtype").unwrap().to_string(), name);
                        }
                        _ => {
                            let _: serde_json::Value = map.next_value()?;
                        }
                    }
                }

                for (key, name) in subtype_names {
                    if let Some(id) = subtype_ids.get(&key) {
                        subtypes.push(Subtype {
                            name,
                            id: id.clone(),
                        });
                    }
                }

                Ok(TypeProperty {
                    name_type: name_type.ok_or_else(|| de::Error::missing_field("NameType"))?,
                    property_type: property_type.ok_or_else(|| de::Error::missing_field("Type"))?,
                    type_id: type_id.ok_or_else(|| de::Error::missing_field("TypeId"))?,
                    subtypes,
                })
            }
        }

        deserializer.deserialize_map(PropertyTypeVisitor)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StatusProperty {
    pub system: String,
    pub en: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PropertyFeatures {
    #[serde(rename = "Category")]
    pub category: Vec<PropertyCategory>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PropertyCategory {
    #[serde(rename = "Type")]
    pub category_type: String,
    #[serde(rename = "Value")]
    pub category_value: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchPropertyPictures {
    #[serde(rename = "Count")]
    pub count: u32,
    #[serde(rename = "Picture")]
    pub picture: Vec<SearchPropertyPicture>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchPropertyPicture {
    #[serde(rename = "Id")]
    pub id: u32,
    #[serde(rename = "PictureURL")]
    pub picture_url: String,
    #[serde(rename = "PictureCaption")]
    pub picture_caption: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PropertyPictures {
    #[serde(rename = "Count")]
    pub count: String,
    #[serde(rename = "Picture")]
    pub picture: Vec<PropertyPicture>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PropertyPicture {
    #[serde(rename = "Id")]
    pub id: u32,
    #[serde(rename = "PictureURL")]
    pub picture_url: String,
}

//....................................................................................................
//.PPPPPPPPP...RRRRRRRRR.......OOOOOO.....PPPPPPPPP...EEEEEEEEEE..RRRRRRRRR....TTTTTTTTTTTYYY....YYY..
//.PPPPPPPPPP..RRRRRRRRRRR...OOOOOOOOOO...PPPPPPPPPP..EEEEEEEEEE..RRRRRRRRRRR..TTTTTTTTTTTYYY....YYY..
//.PPPPPPPPPP..RRRRRRRRRRR..OOOOOOOOOOOO..PPPPPPPPPP..EEEEEEEEEE..RRRRRRRRRRR..TTTTTTTTTTTYYYY..YYYY..
//.PPP....PPPP.RRR.....RRR..OOOO....OOOO..PPP....PPPP.EEE.........RRR.....RRR......TTT.....YYY..YYY...
//.PPP....PPPP.RRR.....RRR..OOO......OOO..PPP....PPPP.EEE.........RRR.....RRR......TTT.....YYYYYYYY...
//.PPPPPPPPPP..RRRRRRRRRRR.ROOO......OOOO.PPPPPPPPPP..EEEEEEEEEE..RRRRRRRRRRR......TTT......YYYYYY....
//.PPPPPPPPPP..RRRRRRRRRR..ROOO......OOOO.PPPPPPPPPP..EEEEEEEEEE..RRRRRRRRRR.......TTT.......YYYY.....
//.PPPPPPPPP...RRRRRRRR....ROOO......OOOO.PPPPPPPPP...EEEEEEEEEE..RRRRRRRR.........TTT.......YYYY.....
//.PPP.........RRR..RRRR....OOO......OOO..PPP.........EEE.........RRR..RRRR........TTT.......YYYY.....
//.PPP.........RRR...RRRR...OOOO....OOOO..PPP.........EEE.........RRR...RRRR.......TTT.......YYYY.....
//.PPP.........RRR....RRRR..OOOOOOOOOOOO..PPP.........EEEEEEEEEEE.RRR....RRRR......TTT.......YYYY.....
//.PPP.........RRR....RRRR...OOOOOOOOOO...PPP.........EEEEEEEEEEE.RRR....RRRR......TTT.......YYYY.....
//.PPP.........RRR.....RRRR....OOOOOO.....PPP.........EEEEEEEEEEE.RRR.....RRRR.....TTT.......YYYY.....

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyResponse {
    pub transaction: PropertyTransaction,
    #[serde(rename = "QueryInfo")]
    pub query_info: PropertyQueryInfo,
    #[serde(rename = "Property")]
    pub property: Property,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyTransaction {
    pub status: String,
    #[serde(rename = "incomingIp")]
    pub incoming_ip: String,
    pub version: String,
    pub subversion: String,
    pub service: String,
    pub datetime: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyQueryInfo {
    #[serde(rename = "ApiId")]
    pub api_id: String,
    #[serde(rename = "QueryId")]
    pub query_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnergyRating {
    #[serde(rename = "CO2Rated")]
    pub co2_rated: String,
    #[serde(rename = "CO2Value")]
    pub co2_value: String,
    #[serde(rename = "EnergyRated")]
    pub energy_rated: String,
    #[serde(rename = "EnergyValue")]
    pub energy_value: String,
    #[serde(rename = "Image")]
    pub image: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicDocument {
    #[serde(rename = "type")]
    pub document_type: String,
    pub order: String,
    #[serde(rename = "URL")]
    pub url: String,
    pub captions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Property {
    #[serde(rename = "Reference")]
    pub reference: String,
    #[serde(rename = "NewDevName")]
    pub newdev_name: String,
    #[serde(rename = "AgencyRef")]
    pub agency_ref: String,
    #[serde(rename = "Status")]
    pub status: StatusProperty,
    #[serde(rename = "Country")]
    pub country: String,
    #[serde(rename = "Province")]
    pub province: String,
    #[serde(rename = "Area")]
    pub area: String,
    #[serde(rename = "Location")]
    pub location: String,
    #[serde(rename = "SubLocation")]
    pub sub_location: String,
    #[serde(rename = "PropertyType")]
    pub property_type: TypeProperty,
    #[serde(rename = "Decree218")]
    pub decree_218: TextOrNum,
    #[serde(rename = "Bedrooms")]
    pub bedrooms: String,
    #[serde(rename = "Bathrooms")]
    pub bathrooms: String,
    #[serde(rename = "Currency")]
    pub currency: String,
    #[serde(rename = "Price")]
    pub price: TextOrNum,
    #[serde(rename = "OriginalPrice")]
    pub original_price: u32,
    #[serde(rename = "Community_Fees_Year")]
    pub community_fee_year: String,
    #[serde(rename = "Basura_Tax_Year")]
    pub basura_tax_year: String,
    #[serde(rename = "IBI_Fees_Year")]
    pub ibi_fee_year: String,
    #[serde(rename = "RentalPeriodShortTerm")]
    pub rental_period_short_term: String,
    #[serde(rename = "RentalPrice1ShortTerm")]
    pub rental_price_1_short_term: u32,
    #[serde(rename = "RentalPrice2ShortTerm")]
    pub rental_price_2_short_term: u32,
    #[serde(rename = "RentalPeriodLongTerm")]
    pub rental_period_long_term: String,
    #[serde(rename = "RentalPrice1LongTerm")]
    pub rental_price_1_long_term: u32,
    #[serde(rename = "RentalPrice2LongTerm")]
    pub rental_price_2_long_term: u32,
    #[serde(rename = "Dimensions")]
    pub dimensions: String,
    #[serde(rename = "Built")]
    pub built: TextOrNum,
    #[serde(rename = "Terrace")]
    pub terrace: TextOrNum,
    #[serde(rename = "GardenPlot")]
    pub garden_plot: TextOrNum,
    #[serde(rename = "IntFloorSpace")]
    pub int_floor_space: TextOrNum,
    #[serde(rename = "Levels")]
    pub levels: String,
    #[serde(rename = "EnergyRating")]
    pub energy_rating: EnergyRating,
    #[serde(rename = "RentalLicenseRef")]
    pub rental_license_ref: String,
    #[serde(rename = "OwnProperty")]
    pub own_property: String,
    #[serde(rename = "VirtualTour")]
    pub virtual_tour: String,
    #[serde(rename = "VideoTour")]
    pub video_tour: String,
    #[serde(rename = "Pool")]
    pub pool: u32,
    #[serde(rename = "Parking")]
    pub parking: u32,
    #[serde(rename = "Garden")]
    pub garden: u32,
    #[serde(rename = "CompletionDate")]
    pub completion_date: String,
    #[serde(rename = "BuiltYear")]
    pub built_year: String,
    #[serde(rename = "PublicDocuments")]
    pub public_documents: Option<PublicDocument>,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "PropertyFeatures")]
    pub property_features: PropertyFeatures,
    #[serde(rename = "Pictures")]
    pub pictures: PropertyPictures,
}
