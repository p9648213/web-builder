use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
    Extension,
};
use axum_csrf::CsrfToken;
use deadpool_postgres::Pool;
use reqwest::StatusCode;

use crate::{
    middlewares::auth::UserId,
    models::{
        error::AppError,
        website::{Website, WebsiteDTO},
    },
    views::builder::{
        data::render_setup_data, template::render_choose_template, website::render_create_website,
    },
};

pub async fn get_section(
    Path(section): Path<String>,
    token: CsrfToken,
    State(pg_pool): State<Pool>,
    Extension(user_id): Extension<UserId>,
) -> Result<impl IntoResponse, AppError> {
    let authenticity_token = token.authenticity_token().unwrap_or("".to_owned());

    let row = Website::get_website_by_user_id(user_id.0, &pg_pool).await?;

    let website: Option<WebsiteDTO> = if let Some(row) = row {
        Some(Website::from_row(row).map_err(|error| {
            tracing::error!("Couldn't convert row to WebsiteDTO: {:?}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?)
    } else {
        None
    };

    match section.as_str() {
        "template" => Ok(Html(render_choose_template(website).into_string()).into_response()),
        "data" => Ok((
            token,
            Html(render_setup_data(authenticity_token).into_string()),
        )
            .into_response()),
        "website" => Ok(
            Html(render_create_website(authenticity_token, website).into_string()).into_response(),
        ),
        _ => Ok(Html("Not found".to_owned()).into_response()),
    }
}
