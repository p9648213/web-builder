use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
    Extension,
};
use axum_csrf::CsrfToken;
use deadpool_postgres::Pool;
use maud::html;
use reqwest::StatusCode;
use serde::Deserialize;

use crate::{
    middlewares::auth::UserId,
    models::{error::AppError, website::Website, website_setting::WebsiteSetting},
    views::builder::{
        data::render_setup_data_page, edit::render_edit_style_page,
        template::render_website_template_page, website::render_create_website_page,
    },
};

pub enum BuilderStyle {
    Header,
    Footer,
    Home,
    SearchResult,
    PropertyDetail,
    Contact,
}

pub enum BuilderSection {
    ChooseStyle,
    Branding,
    Content,
    AdvancedSetting,
}

#[derive(Deserialize)]
pub struct EditParams {
    pub website_id: i32,
    pub section: String,
    pub sub_section: String,
}

pub async fn get_create_website_page(token: CsrfToken) -> impl IntoResponse {
    (token, Html(render_create_website_page().into_string()))
}

pub async fn get_select_template_page(token: CsrfToken) -> impl IntoResponse {
    (token, Html(render_website_template_page().into_string()))
}

pub async fn get_setup_data_page(token: CsrfToken) -> impl IntoResponse {
    (token, Html(render_setup_data_page().into_string()))
}

pub async fn get_edit_page(
    Path(EditParams {
        website_id,
        section,
        sub_section,
    }): Path<EditParams>,
    token: CsrfToken,
    State(pg_pool): State<Pool>,
    Extension(user_id): Extension<UserId>,
) -> Result<impl IntoResponse, AppError> {
    let authenticity_token = token.authenticity_token().unwrap_or("".to_owned());

    let row =
        Website::get_website_by_user_id_and_website_id(user_id.0, website_id, &pg_pool, vec!["id"])
            .await?;

    if row.is_none() {
        tracing::error!(
            "No website with id {} and user_id {} is found",
            website_id,
            user_id.0
        );
        return Err(AppError::new(StatusCode::NOT_FOUND, "Not Found"));
    }

    let html = match section.as_str() {
        "style" => {
            match sub_section.as_str() {
                "header" => {
                    let row = WebsiteSetting::get_website_setting_by_website_id(
                        website_id,
                        &pg_pool,
                        vec!["id","header_theme"],
                    )
                    .await?;

                    let website_setting = if let Some(row) = row {
                        WebsiteSetting::try_from(row)
                    } else {
                        tracing::error!(
                            "No website_setting with website_id {} is found",
                            website_id
                        );
                        return Err(AppError::new(
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "Server Error",
                        ));
                    };

                    let header_theme = website_setting.header_theme.ok_or_else(|| {
                        tracing::error!("No header_theme column or value is null");
                        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
                    })?;

                    let id = website_setting.id.ok_or_else(|| {
                        tracing::error!("No id column or value is null");
                        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
                    })?;

                    html! {
                        (render_edit_style_page(BuilderSection::ChooseStyle, BuilderStyle::Header, header_theme, id, user_id.0, authenticity_token))
                    }
                }
                _ => html! {
                    "Not Found"
                },
            }
        }
        .into_string(),
        _ => html! {
            "Not Found"
        }
        .into_string(),
    };

    Ok((token, Html(html)))
}
