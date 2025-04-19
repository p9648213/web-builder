use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::Html;
use deadpool_postgres::Pool;
use maud::html;
use serde::Deserialize;

use crate::models::error::AppError;
use crate::models::function_params::RenderPropertyGrid;
use crate::models::rso_data::{
    HotPropertyParams, LocationParams, PropertyParams, PropertyTypeParams, RsoData,
    SearchResultParams,
};
use crate::views::real_estate::home;
use crate::views::real_estate::property_details;
use crate::views::real_estate::search_result;

use super::pages::{PropertyQuery, SearchQuery};

#[derive(Deserialize, Debug)]
pub struct HotPropertyQuery {
    pub theme: Option<i32>,
}

//..............................................................................................
//.LLL...........OOOOOO........CCCCCC.......AAAA.....ATTTTTTTTTTTII.....OOOOOO.....ONN....NNNN..
//.LLL.........OOOOOOOOOO....CCCCCCCCC......AAAAA....ATTTTTTTTTTTII...OOOOOOOOO....ONNN...NNNN..
//.LLL........OOOOOOOOOOOO..CCCCCCCCCCC.....AAAAA....ATTTTTTTTTTTII..IOOOOOOOOOO...ONNNN..NNNN..
//.LLL........OOOO....OOOO..CCCC...CCCC....AAAAAA.......TTTT....TII..IOOO...OOOOO..ONNNN..NNNN..
//.LLL........OOO......OOO..CCC.....CC.....AAAAAAA......TTTT....TII.IIOO......OOO..ONNNNN.NNNN..
//.LLL.......LOOO......OOOOOCCC...........AAAA.AAA......TTTT....TII.IIOO......OOO..ONNNNN.NNNN..
//.LLL.......LOOO......OOOOOCCC...........AAA..AAAA.....TTTT....TII.IIOO......OOO..ONNNNNNNNNN..
//.LLL.......LOOO......OOOOOCCC...........AAAAAAAAA.....TTTT....TII.IIOO......OOO..ONN.NNNNNNN..
//.LLL........OOO......OOO..CCC.....CC...AAAAAAAAAA.....TTTT....TII.IIOO.....OOOO..ONN..NNNNNN..
//.LLL........OOOO....OOOO..CCCC...CCCC..AAAAAAAAAAA....TTTT....TII..IOOO...OOOOO..ONN..NNNNNN..
//.LLLLLLLLLL.OOOOOOOOOOOO..CCCCCCCCCCC..AAA.....AAA....TTTT....TII..IOOOOOOOOOO...ONN...NNNNN..
//.LLLLLLLLLL..OOOOOOOOOO....CCCCCCCCC..CAAA.....AAAA...TTTT....TII...OOOOOOOOO....ONN...NNNNN..
//.LLLLLLLLLL....OOOOOO........CCCCCC...CAA......AAAA...TTTT....TII.....OOOOOO.....ONN....NNNN..
//..............................................................................................

pub async fn get_locations(State(pg_pool): State<Pool>) -> Result<Html<String>, AppError> {
    let row = RsoData::get_rso_data_by_user_id(
        1,
        &pg_pool,
        vec!["filter_id_sale", "identifier_id", "api_key"],
    )
    .await?;

    if let Some(row) = row {
        let rso_data = RsoData::try_from(&row, None);

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
            (home::render_location_selection_drop_down_1(location.location_data.province_area, "All"))
            (home::render_selection_label_1("All", "location-label"))
        }
        .into_string();

        Ok(Html(html))
    } else {
        tracing::error!("No rso data found for user id 1");
        Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Server Error",
        ))
    }
}

//....................................................................................................
//.PPPPPPPPP...RRRRRRRRR.......OOOOOO.....PPPPPPPPP...TTTTTTTTTTTYYY....YYYY.PPPPPPPPP...EEEEEEEEEE...
//.PPPPPPPPPP..RRRRRRRRRRR...OOOOOOOOOO...PPPPPPPPPP..TTTTTTTTTTTYYY....YYY..PPPPPPPPPP..EEEEEEEEEE...
//.PPPPPPPPPP..RRRRRRRRRRR..OOOOOOOOOOOO..PPPPPPPPPP..TTTTTTTTTTTYYYY..YYYY..PPPPPPPPPP..EEEEEEEEEE...
//.PPP....PPPP.RRR.....RRR..OOOO....OOOO..PPP....PPPP.....TTT.....YYYY.YYY...PPP....PPPP.EEE..........
//.PPP....PPPP.RRR.....RRR..OOO......OOO..PPP....PPPP.....TTT.....YYYYYYYY...PPP....PPPP.EEE..........
//.PPPPPPPPPP..RRRRRRRRRRR.ROOO......OOOO.PPPPPPPPPP......TTT......YYYYYY....PPPPPPPPPP..EEEEEEEEEE...
//.PPPPPPPPPP..RRRRRRRRRR..ROOO......OOOO.PPPPPPPPPP......TTT.......YYYY.....PPPPPPPPPP..EEEEEEEEEE...
//.PPPPPPPPP...RRRRRRRR....ROOO......OOOO.PPPPPPPPP.......TTT.......YYYY.....PPPPPPPPP...EEEEEEEEEE...
//.PPP.........RRR..RRRR....OOO......OOO..PPP.............TTT.......YYYY.....PPP.........EEE..........
//.PPP.........RRR...RRRR...OOOO....OOOO..PPP.............TTT.......YYYY.....PPP.........EEE..........
//.PPP.........RRR....RRRR..OOOOOOOOOOOO..PPP.............TTT.......YYYY.....PPP.........EEEEEEEEEEE..
//.PPP.........RRR....RRRR...OOOOOOOOOO...PPP.............TTT.......YYYY.....PPP.........EEEEEEEEEEE..
//.PPP.........RRR.....RRRR....OOOOOO.....PPP.............TTT.......YYYY.....PPP.........EEEEEEEEEEE..
//....................................................................................................

pub async fn get_property_types(State(pg_pool): State<Pool>) -> Result<Html<String>, AppError> {
    let row = RsoData::get_rso_data_by_user_id(
        1,
        &pg_pool,
        vec!["filter_id_sale", "identifier_id", "api_key"],
    )
    .await?;

    if let Some(row) = row {
        let rso_data = RsoData::try_from(&row, None);

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
            (home::render_property_types_selection_drop_down_1(property_type.property_types.property_type))
            (home::render_selection_label_1("All", "property-types-label"))
        }
        .into_string();

        Ok(Html(html))
    } else {
        tracing::error!("No rso data found for user id 1");
        Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Server Error",
        ))
    }
}

//...........................................................................................
//.HHH.....HHH.....OOOOOO.....TTTTTTTTTTTPPPPPPPPP...RRRRRRRRR.......OOOOOO.....PPPPPPPPP....
//.HHH.....HHH...OOOOOOOOOO...TTTTTTTTTTTPPPPPPPPPP..RRRRRRRRRRR...OOOOOOOOOO...PPPPPPPPPP...
//.HHH.....HHH..OOOOOOOOOOOO..TTTTTTTTTTTPPPPPPPPPP..RRRRRRRRRRR..OOOOOOOOOOOO..PPPPPPPPPP...
//.HHH.....HHH..OOOO....OOOO......TTT....PPP....PPPP.RRR.....RRR..OOOO....OOOO..PPP....PPPP..
//.HHH.....HHH..OOO......OOO......TTT....PPP....PPPP.RRR.....RRR..OOO......OOO..PPP....PPPP..
//.HHHHHHHHHHH.HOOO......OOOO.....TTT....PPPPPPPPPP..RRRRRRRRRRR.ROOO......OOOO.PPPPPPPPPP...
//.HHHHHHHHHHH.HOOO......OOOO.....TTT....PPPPPPPPPP..RRRRRRRRRR..ROOO......OOOO.PPPPPPPPPP...
//.HHHHHHHHHHH.HOOO......OOOO.....TTT....PPPPPPPPP...RRRRRRRR....ROOO......OOOO.PPPPPPPPP....
//.HHH.....HHH..OOO......OOO......TTT....PPP.........RRR..RRRR....OOO......OOO..PPP..........
//.HHH.....HHH..OOOO....OOOO......TTT....PPP.........RRR...RRRR...OOOO....OOOO..PPP..........
//.HHH.....HHH..OOOOOOOOOOOO......TTT....PPP.........RRR....RRRR..OOOOOOOOOOOO..PPP..........
//.HHH.....HHH...OOOOOOOOOO.......TTT....PPP.........RRR....RRRR...OOOOOOOOOO...PPP..........
//.HHH.....HHH.....OOOOOO.........TTT....PPP.........RRR.....RRRR....OOOOOO.....PPP..........
//...........................................................................................

pub async fn get_hot_properties(
    hot_property_query: Query<HotPropertyQuery>,
    State(pg_pool): State<Pool>,
) -> Result<Html<String>, AppError> {
    let hot_property_theme = hot_property_query.theme.unwrap_or(1);

    let row = RsoData::get_rso_data_by_user_id(
        1,
        &pg_pool,
        vec!["filter_id_featured", "identifier_id", "api_key"],
    )
    .await?;

    if let Some(row) = row {
        let rso_data = RsoData::try_from(&row, None);

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
            @match hot_property_theme {
                1 => (home::render_hot_properties_slider_1(property_response.property)),
                2 => (home::render_hot_properties_slider_2(property_response.property)),
                3 => (home::render_hot_properties_slider_3(property_response.property)),
                4 => (home::render_hot_properties_slider_4(property_response.property)),
                _ => (home::render_hot_properties_slider_1(property_response.property))
            }

        }
        .into_string();

        Ok(Html(html))
    } else {
        tracing::error!("No rso data found for user id 1");
        Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Server Error",
        ))
    }
}

//.....................................................................................................
//.PPPPPPPPP...RRRRRRRRR.......OOOOOO.....PPPPPPPPP...EEEEEEEEEE..RRRRRRRRR....TTTTTTTTTTTYYY....YYYY..
//.PPPPPPPPPP..RRRRRRRRRRR...OOOOOOOOOO...PPPPPPPPPP..EEEEEEEEEE..RRRRRRRRRRR..TTTTTTTTTTTYYY....YYY...
//.PPPPPPPPPP..RRRRRRRRRRR..OOOOOOOOOOOO..PPPPPPPPPP..EEEEEEEEEE..RRRRRRRRRRR..TTTTTTTTTTTYYYY..YYYY...
//.PPP....PPPP.RRR.....RRR..OOOO....OOOO..PPP....PPPP.EEE.........RRR.....RRR......TTT.....YYYY.YYY....
//.PPP....PPPP.RRR.....RRR..OOO......OOO..PPP....PPPP.EEE.........RRR.....RRR......TTT.....YYYYYYYY....
//.PPPPPPPPPP..RRRRRRRRRRR.ROOO......OOOO.PPPPPPPPPP..EEEEEEEEEE..RRRRRRRRRRR......TTT......YYYYYY.....
//.PPPPPPPPPP..RRRRRRRRRR..ROOO......OOOO.PPPPPPPPPP..EEEEEEEEEE..RRRRRRRRRR.......TTT.......YYYY......
//.PPPPPPPPP...RRRRRRRR....ROOO......OOOO.PPPPPPPPP...EEEEEEEEEE..RRRRRRRR.........TTT.......YYYY......
//.PPP.........RRR..RRRR....OOO......OOO..PPP.........EEE.........RRR..RRRR........TTT.......YYYY......
//.PPP.........RRR...RRRR...OOOO....OOOO..PPP.........EEE.........RRR...RRRR.......TTT.......YYYY......
//.PPP.........RRR....RRRR..OOOOOOOOOOOO..PPP.........EEEEEEEEEEE.RRR....RRRR......TTT.......YYYY......
//.PPP.........RRR....RRRR...OOOOOOOOOO...PPP.........EEEEEEEEEEE.RRR....RRRR......TTT.......YYYY......
//.PPP.........RRR.....RRRR....OOOOOO.....PPP.........EEEEEEEEEEE.RRR.....RRRR.....TTT.......YYYY......
//.....................................................................................................

pub async fn get_property(
    State(pg_pool): State<Pool>,
    property_query: Query<PropertyQuery>,
) -> Result<Html<String>, AppError> {
    if property_query.0.id.is_none() || property_query.0.listing_type.is_none() {
        return Ok(Html("Property not found".to_string()));
    }

    let theme = property_query.0.theme.unwrap_or(1);

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
        let rso_data = RsoData::try_from(&row, None);

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
            "resales" | "new-development" => rso_data.filter_id_sale.ok_or_else(|| {
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
            @match theme {
                1 => (property_details::render_detail_1(&property_response.property, listing_type.as_str())),
                2 => (property_details::render_detail_2(&property_response.property, listing_type.as_str())),
                3 => (property_details::render_detail_3(&property_response.property, listing_type.as_str())),
                4 => (property_details::render_detail_4(&property_response.property, listing_type.as_str())),
                _ => (property_details::render_detail_1(&property_response.property, listing_type.as_str()))
            }
        }
        .into_string();

        Ok(Html(html))
    } else {
        tracing::error!("No rso data found for user id 1");
        Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Server Error",
        ))
    }
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
//..............................................................................

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
        let rso_data = RsoData::try_from(&row, None);

        let p1 = rso_data.identifier_id.ok_or_else(|| {
            tracing::error!("No identifier_id column or value is null");
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
        })?;

        let p2 = rso_data.api_key.ok_or_else(|| {
            tracing::error!("No api_key column or value is null");
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
        })?;

        let page_no = search_query.0.page.unwrap_or(1);

        let listing_type = search_query.0.listing_type.unwrap_or("resales".to_string());

        let mut province = search_query.0.province.unwrap_or("".to_string());

        let location = search_query.0.location.unwrap_or("".to_string());

        if province == "All" {
            province = "".to_string();
        }

        let property_types = search_query.0.property_type.unwrap_or("".to_string());

        let mut min_price = search_query.0.min_price.unwrap_or("0".to_string());

        if min_price == "0" {
            min_price = "".to_string();
        }

        let mut max_price = search_query.0.max_price.unwrap_or("0".to_string());

        if max_price == "0" {
            max_price = "".to_string();
        }

        let p_agency_filterid = match listing_type.as_str() {
            "resales" | "new-development" => rso_data.filter_id_sale.ok_or_else(|| {
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
            p_province: province.to_owned(),
            p_location: location.to_owned(),
            p_property_types: property_types.to_owned(),
            p_min: min_price.to_owned(),
            p_max: max_price.to_owned(),
        };

        let search_response = RsoData::get_rso_search_result(search_result_params).await?;

        let search_theme = search_query.0.theme.unwrap_or(1); 

        let property_grid_params = RenderPropertyGrid {
            properties: &search_response.property,
            property_count: search_response.query_info.property_count,
            properties_per_page: search_response.query_info.properties_per_page,
            page_no: search_response.query_info.current_page,
            listing_type: &listing_type,
            province: &province,
            location: &location,
            property_type: &property_types,
            min_price: &min_price,
            max_price: &max_price,
        };

        let html = html! {
            @match search_theme {
                1 => {
                    (search_result::render_property_grids_1(property_grid_params))
                }
                2 => {
                    (search_result::render_property_grids_2(property_grid_params))
                }
                3 => {
                    (search_result::render_property_grids_3(property_grid_params))
                }
                4 => {
                    (search_result::render_property_grids_4(property_grid_params))
                }
                _ => {}
            }
        }
        .into_string();

        Ok(Html(html))
    } else {
        tracing::error!("No rso data found for user id 1");
        Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Server Error",
        ))
    }
}
