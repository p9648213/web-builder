use axum::{
    extract::{Path, Query, Request, State},
    http::HeaderValue,
    response::Html,
};
use deadpool_postgres::Pool;
use maud::html;
use reqwest::StatusCode;

use crate::{
    models::{error::AppError, website_setting_website::WebsiteSettingWebsite},
    views::real_estate::{home, property_details, search_result, shared},
};

use super::pages::PropertyQuery;

pub async fn get_section(
    Path(section): Path<String>,
    property_query: Query<PropertyQuery>,
    State(pg_pool): State<Pool>,
    request: Request,
) -> Result<Html<String>, AppError> {
    match section.as_str() {
        "home" => {
            let html = html! {
                (home::render_home_banner())
                (home::render_home_search_box())
                (home::render_hot_properties())
                (home::render_our_services())
                (home::render_testimonial())
                (shared::render_contact())
            };

            Ok(Html(html.into_string()))
        }
        "search-results" => {
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

            let row = WebsiteSettingWebsite::get_website_setting_by_domain(
                host,
                &pg_pool,
                vec!["search_theme"],
            )
            .await?;

            if let Some(row) = row {
                let website_setting = WebsiteSettingWebsite::try_from(row);

                let search_theme = website_setting.search_theme.ok_or_else(|| {
                    tracing::error!("No search_theme column or value is null");
                    AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
                })?;

                let html = html! {
                    div id="search-section" class="min-h-screen invisible" {
                        @match search_theme {
                            1 => {
                                (search_result::render_search_box_1())
                                (search_result::render_search_result_1(None))
                            }
                            2 => {
                                (search_result::render_search_box_2())
                                (search_result::render_search_result_2(None))
                            }
                            3 => {
                                (search_result::render_search_box_3())
                                (search_result::render_search_result_3(None))
                            }
                            4 => {
                                (search_result::render_search_box_4())
                                (search_result::render_search_result_4(None))
                            }
                            _ => {
                                (search_result::render_search_box_1())
                                (search_result::render_search_result_1(None))
                            }
                        }
                    }
                    (shared::render_contact())
                };

                Ok(Html(html.into_string()))
            } else {
                Err(AppError::new(StatusCode::NOT_FOUND, "Domain not found"))
            }
        }
        "property" => {
            let html = html! {
                (property_details::render_property_details(property_query.0))
                (shared::render_contact())
            };

            Ok(Html(html.into_string()))
        }
        _ => Ok(Html("Not found".to_owned())),
    }
}
