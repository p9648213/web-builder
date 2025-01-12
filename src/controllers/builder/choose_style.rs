use axum::{
    extract::{Path, State},
    response::Html,
};
use axum_csrf::CsrfToken;
use deadpool_postgres::Pool;
use maud::html;
use reqwest::StatusCode;
use serde::Deserialize;

use crate::{
    models::{error::AppError, website_setting::WebsiteSetting},
    views::builder::edit::{
        render_preview_image, render_style_selection, FOOTER_LAYOUT, HEADER_LAYOUT,
    },
};

#[derive(Deserialize, Debug)]
pub struct EditChooseStyleParams {
    pub user_id: i32,
    pub setting_id: i32,
    pub part: String,
    pub theme: i32,
}

pub async fn update_style(
    Path(EditChooseStyleParams {
        user_id,
        setting_id,
        part,
        theme,
    }): Path<EditChooseStyleParams>,
    token: CsrfToken,
    State(pg_pool): State<Pool>,
) -> Result<Html<String>, AppError> {
    let authenticity_token = token.authenticity_token().unwrap_or("".to_owned());

    let row = WebsiteSetting::get_website_setting_by_id_and_user_id(
        setting_id,
        user_id,
        &pg_pool,
        vec!["id"],
    )
    .await?;

    if row.is_none() {
        tracing::error!(
            "No website_setting with id {} and user_id {} is found",
            setting_id,
            user_id
        );
        return Err(AppError::new(StatusCode::NOT_FOUND, "Not Found"));
    }

    match part.as_str() {
        "header" => {
            let result =
                WebsiteSetting::update_header_theme_by_id(setting_id, theme, &pg_pool).await?;

            if result == 0 {
                tracing::error!("Error while updating header theme. No rows were affected");
                return Err(AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Server error",
                ));
            }

            let html = html! {
              (render_style_selection(&HEADER_LAYOUT, "header", theme, setting_id, user_id, authenticity_token))
              (render_preview_image(&HEADER_LAYOUT, theme, Some("outerHTML")))
            };

            Ok(Html(html.into_string()))
        }
        "footer" => {
            let result =
                WebsiteSetting::update_footer_theme_by_id(setting_id, theme, &pg_pool).await?;

            if result == 0 {
                tracing::error!("Error while updating footer theme. No rows were affected");
                return Err(AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Server error",
                ));
            }

            let html = html! {
              (render_style_selection(&FOOTER_LAYOUT, "footer", theme, setting_id, user_id, authenticity_token))
              (render_preview_image(&FOOTER_LAYOUT, theme, Some("outerHTML")))
            };

            Ok(Html(html.into_string()))
        }
        _ => {
            return Err(AppError::new(StatusCode::NOT_FOUND, "Not Found"));
        }
    }
}
