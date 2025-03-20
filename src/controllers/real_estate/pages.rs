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
    pub province: Option<String>,
    pub location: Option<String>,
    pub property_type: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct PropertyQuery {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub listing_type: Option<String>,
    pub theme: Option<i32>,
}

//........................................................
//.HHH.....HHH.....OOOOOO.....MMMMM...MMMMM..EEEEEEEEEE...
//.HHH.....HHH...OOOOOOOOOO...MMMMM...MMMMM..EEEEEEEEEE...
//.HHH.....HHH..OOOOOOOOOOOO..MMMMM...MMMMM..EEEEEEEEEE...
//.HHH.....HHH..OOOO....OOOO..MMMMM...MMMMM..EEE..........
//.HHH.....HHH..OOO......OOO..MMMMMM.MMMMMM..EEE..........
//.HHHHHHHHHHH.HOOO......OOOO.MMMMMM.MMMMMM..EEEEEEEEEE...
//.HHHHHHHHHHH.HOOO......OOOO.MMMMMM.MMMMMM..EEEEEEEEEE...
//.HHHHHHHHHHH.HOOO......OOOO.MMMMMMMMMMMMM..EEEEEEEEEE...
//.HHH.....HHH..OOO......OOO..MMM.MMMMM.MMM..EEE..........
//.HHH.....HHH..OOOO....OOOO..MMM.MMMMM.MMM..EEE..........
//.HHH.....HHH..OOOOOOOOOOOO..MMM.MMMMM.MMM..EEEEEEEEEEE..
//.HHH.....HHH...OOOOOOOOOO...MMM..MMMM.MMM..EEEEEEEEEEE..
//.HHH.....HHH.....OOOOOO.....MMM..MMM..MMM..EEEEEEEEEEE..
//........................................................

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
        Some(vec!["home_theme", "header_theme", "footer_theme"]),
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

        let home_theme = website_setting_website
            .website_setting
            .home_theme
            .ok_or_else(|| {
                tracing::error!("No home_theme column or value is null");
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
            })?;

        Ok(Html(
            render_home_page(home_theme, header_theme, footer_theme).into_string(),
        ))
    } else {
        Err(AppError::new(StatusCode::NOT_FOUND, "Domain not found"))
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
            render_search_result_page(search_query.0, header_theme, footer_theme, search_theme)?
                .into_string(),
        ))
    } else {
        Err(AppError::new(StatusCode::NOT_FOUND, "Domain not found"))
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

//.........................................................................................
//.....CCCCCC.......OOOOOO.....NNNN....NNN..TTTTTTTTTTA..AAAA.........CCCCC.....TTTTTTTTT..
//...CCCCCCCCC....OOOOOOOOOO...NNNN....NNN..TTTTTTTTTTA..AAAA.......CCCCCCCCC...TTTTTTTTT..
//..CCCCCCCCCCC..OOOOOOOOOOOO..NNNNN...NNN..TTTTTTTTTTA.AAAAAA.....CCCCCCCCCCC..TTTTTTTTT..
//..CCCC...CCCC..OOOO....OOOO..NNNNN...NNN......TTT.....AAAAAA.....CCCC...CCCC.....TTTT....
//..CCC.....CC...OOO......OOO..NNNNNN..NNN......TTT.....AAAAAAA...ACCC.....CC......TTTT....
//.CCCC.........OOOO......OOOO.NNNNNNN.NNN......TTT....AAAAAAAA...ACCC.............TTTT....
//.CCCC.........OOOO......OOOO.NNN.NNN.NNN......TTT....AAA..AAA...ACCC.............TTTT....
//.CCCC.........OOOO......OOOO.NNN.NNNNNNN......TTT...AAAAAAAAAA..ACCC.............TTTT....
//..CCC.....CC...OOO......OOO..NNN..NNNNNN......TTT...AAAAAAAAAA..ACCC.....CC......TTTT....
//..CCCC...CCCC..OOOO....OOOO..NNN..NNNNNN......TTT...AAAAAAAAAAA..CCCC...CCCC.....TTTT....
//..CCCCCCCCCCC..OOOOOOOOOOOO..NNN...NNNNN......TTT..TAAA....AAAA..CCCCCCCCCCC.....TTTT....
//...CCCCCCCCC....OOOOOOOOOO...NNN....NNNN......TTT..TAA......AAA...CCCCCCCCC......TTTT....
//.....CCCCCC.......OOOOOO.....NNN....NNNN......TTT..TAA......AAAA....CCCCCC.......TTTT....
//.........................................................................................

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
