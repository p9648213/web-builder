use axum::{
    response::{Html, IntoResponse},
    Form,
};
use reqwest::StatusCode;
use serde::Deserialize;

use crate::models::error::AppError;

#[derive(Deserialize, Debug)]
pub struct DataSourceForm {
    pub url: String,
}

pub async fn get_data_source(
    Form(form): Form<DataSourceForm>,
) -> Result<impl IntoResponse, AppError> {
    let resp = reqwest::get(form.url).await.map_err(|err| {
        tracing::error!("Failed to fetch url: {}", err);
        AppError::new(StatusCode::BAD_REQUEST, "Failed to fetch url")
    })?;

    let text = resp.text().await.map_err(|err| {
        tracing::error!("Failed to convert : {}", err);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
    })?;

    Ok(Html(
        maud::html! {
          script src="/assets/js/builder/data-source.js" defer="defer" {}
          div id="data-json" class="mt-12 max-h-96 text-wrap overflow-auto" {(text)}
          div class="mt-12" {
            h1 {"Setup display product fields: "}
          }
        }
        .into_string(),
    ))
}
