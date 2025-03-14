use axum::{
    extract::{Path, Query, Request, State},
    http::HeaderValue,
    response::Html,
};
use deadpool_postgres::Pool;
use maud::html;
use reqwest::StatusCode;

use crate::{
    models::{error::AppError, website_setting_website::WebsiteJoinWebsiteSetting},
    views::real_estate::{contact, home, property_details, search_result, shared},
};

use super::pages::PropertyQuery;
use super::pages::SearchQuery;

pub async fn get_section(
    Path(section): Path<String>,
    State(pg_pool): State<Pool>,
    request: Request,
) -> Result<Html<String>, AppError> {
    match section.as_str() {
        "home" => {
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
                Some(vec!["home_theme"]),
                Some("w"),
                Some("s"),
            )
            .await?;

            if let Some(row) = row {
                let website_setting_website = WebsiteJoinWebsiteSetting::try_from(&row, "w_", "s_");

                let home_theme = website_setting_website
                    .website_setting
                    .home_theme
                    .ok_or_else(|| {
                        tracing::error!("No home_theme column or value is null");
                        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
                    })?;

                let html = html! {
                    title {
                        "Home"
                    }
                    @match home_theme {
                        1 => {
                            (home::render_home_banner_1())
                            (home::render_hot_properties_1())
                            (home::render_our_services_1())
                            (home::render_testimonial_1())
                        },
                        2 => {
                            (home::render_home_banner_2())
                            (home::render_hot_properties_2())
                            (home::render_our_services_2())
                            (home::render_testimonial_2())
                        },
                        3 => {
                            (home::render_home_banner_3())
                            (home::render_hot_properties_3())
                            (home::render_our_services_3())
                            (home::render_testimonial_3())
                        },
                        4 => {
                            (home::render_home_banner_4())
                            (home::render_hot_properties_4())
                            (home::render_our_services_4())
                            (home::render_testimonial_4())
                        },
                        _ => {
                            (home::render_home_banner_1())
                            (home::render_hot_properties_1())
                            (home::render_our_services_1())
                            (home::render_testimonial_1())
                        }
                    }
                    (shared::render_contact())
                };

                Ok(Html(html.into_string()))
            } else {
                Err(AppError::new(StatusCode::NOT_FOUND, "Domain not found"))
            }
        }
        "search-results" => {
            let default_host = HeaderValue::from_static("");

            let uri = request.uri();

            let search_query: Query<SearchQuery> = Query::try_from_uri(uri).map_err(|error| {
                tracing::error!("Failed to extract search query: {}", error);
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
            })?;

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
                Some(vec!["search_theme"]),
                Some("w"),
                Some("s"),
            )
            .await?;

            if let Some(row) = row {
                let website_setting_website = WebsiteJoinWebsiteSetting::try_from(&row, "w_", "s_");

                let search_theme = website_setting_website
                    .website_setting
                    .search_theme
                    .ok_or_else(|| {
                        tracing::error!("No search_theme column or value is null");
                        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
                    })?;

                let html = html! {
                    title {
                        "Search Result"
                    }
                    div id="search-section" class="invisible flex flex-col items-center min-h-screen" {
                        @match search_theme {
                            1 => {
                                (search_result::render_search_box_1(&search_query.0))
                                (search_result::render_search_result_1(&search_query.0)?)
                            }
                            2 => {
                                div class="relative flex justify-between gap-10 mt-15 px-5 pb-30 w-full max-w-360" {
                                    (search_result::render_search_box_2())
                                    (search_result::render_search_result_2(None))
                                }
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
                                (search_result::render_search_box_1(&search_query.0))
                                (search_result::render_search_result_1(&search_query.0)?)
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
            let default_host = HeaderValue::from_static("");

            let uri = request.uri();

            let property_query: Query<PropertyQuery> =
                Query::try_from_uri(uri).map_err(|error| {
                    tracing::error!("Failed to extract property query: {}", error);
                    AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
                })?;

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
                Some(vec!["property_theme"]),
                Some("w"),
                Some("s"),
            )
            .await?;

            if let Some(row) = row {
                let website_setting_website = WebsiteJoinWebsiteSetting::try_from(&row, "w_", "s_");

                let property_theme = website_setting_website
                    .website_setting
                    .property_theme
                    .ok_or_else(|| {
                        tracing::error!("No property_theme column or value is null");
                        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
                    })?;

                let html = html! {
                    title {
                        "Property Details"
                    }
                    div id="property-section" class="invisible min-h-screen" {
                        @match property_theme {
                            1 => (property_details::render_property_details_1(&property_query.0)),
                            2 => (property_details::render_property_details_2(&property_query.0)),
                            3 => (property_details::render_property_details_3(&property_query.0)),
                            4 => (property_details::render_property_details_4(&property_query.0)),
                            _ => (property_details::render_property_details_1(&property_query.0))
                        }
                    }
                    (shared::render_contact())
                };

                Ok(Html(html.into_string()))
            } else {
                Err(AppError::new(StatusCode::NOT_FOUND, "Domain not found"))
            }
        }
        "contact" => {
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
                Some(vec!["contact_theme"]),
                Some("w"),
                Some("s"),
            )
            .await?;

            if let Some(row) = row {
                let website_setting_website = WebsiteJoinWebsiteSetting::try_from(&row, "w_", "s_");

                let contact_theme = website_setting_website
                    .website_setting
                    .contact_theme
                    .ok_or_else(|| {
                        tracing::error!("No contact_theme column or value is null");
                        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
                    })?;

                let html = html! {
                    title {
                        "Contact"
                    }
                    div id="contact-section" class="invisible min-h-screen" {
                        @match contact_theme {
                            1 => (contact::render_contact_1()),
                            2 => (contact::render_contact_2()),
                            3 => (contact::render_contact_3()),
                            4 => (contact::render_contact_4()),
                            _ => (contact::render_contact_1())
                        }
                    }
                };

                Ok(Html(html.into_string()))
            } else {
                Err(AppError::new(StatusCode::NOT_FOUND, "Domain not found"))
            }
        }
        _ => Ok(Html("Not found".to_owned())),
    }
}
