use axum::{extract::State, response::IntoResponse, Extension, Form};
use deadpool_postgres::Pool;
use reqwest::StatusCode;
use serde::Deserialize;

use crate::{
    middlewares::auth::UserId,
    models::{error::AppError, rso_data::RsoData},
};

#[derive(Deserialize, Debug)]
pub struct DataSourceForm {
    pub identifier_id: i32,
    pub api_key: String,
    pub filter_id_sale: i32,
    pub filter_id_long: i32,
    pub filter_id_short: i32,
    pub filter_id_featured: i32,
}

#[derive(Deserialize, Debug)]
pub struct RsoStatusForm {
    pub rso_data_status: Option<bool>,
}

pub async fn create_data_source(
    Extension(user_id): Extension<UserId>,
    State(pg_pool): State<Pool>,
    Form(form): Form<DataSourceForm>,
) -> Result<impl IntoResponse, AppError> {
    let row = RsoData::get_rso_data_by_user_id(user_id.0, &pg_pool, vec!["id"]).await?;

    if row.is_some() {
        let updated_rso_data = RsoData {
            id: None,
            user_id: None,
            identifier_id: Some(form.identifier_id),
            api_key: Some(form.api_key),
            filter_id_sale: Some(form.filter_id_sale),
            filter_id_long: Some(form.filter_id_long),
            filter_id_short: Some(form.filter_id_short),
            filter_id_featured: Some(form.filter_id_featured),
        };

        let result =
            RsoData::update_rso_data_by_user_id(user_id.0, &updated_rso_data, &pg_pool).await?;

        if result == 0 {
            tracing::error!("Error while updating rso data. No rows were affected");
            return Err(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Server error",
            ));
        }

        Ok(())
    } else {
        let rso_data = RsoData {
            id: None,
            user_id: Some(user_id.0),
            identifier_id: Some(form.identifier_id),
            api_key: Some(form.api_key),
            filter_id_sale: Some(form.filter_id_sale),
            filter_id_long: Some(form.filter_id_long),
            filter_id_short: Some(form.filter_id_short),
            filter_id_featured: Some(form.filter_id_featured),
        };

        let result = RsoData::create_rso_data_by_user_id(user_id.0, &rso_data, &pg_pool).await?;

        if result == 0 {
            tracing::error!("Error while creating rso data. No rows were affected");
            return Err(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Server error",
            ));
        }

        Ok(())
    }
}

pub async fn update_rso_status(
    Extension(user_id): Extension<UserId>,
    State(pg_pool): State<Pool>,
    Form(form): Form<RsoStatusForm>,
) -> Result<impl IntoResponse, AppError> {
    let row = RsoData::get_rso_data_by_user_id(user_id.0, &pg_pool, vec!["id"]).await?;

    if row.is_some() {
        let status = form.rso_data_status.is_some();

        let result =
            RsoData::update_status_rso_data_by_user_id(user_id.0, status, &pg_pool).await?;

        if result == 0 {
            tracing::error!("Error while updating rso data status. No rows were affected");
            return Err(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Server error",
            ));
        }

        Ok(())
    } else {
        Ok(())
    }
}
