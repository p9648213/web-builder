use axum::http::StatusCode;
use deadpool_postgres::Pool;
use tokio_postgres::{types::ToSql, Row};

use crate::models::error::AppError;

pub async fn query(
    query: &str,
    params: &[&(dyn ToSql + Sync)],
    pool: &Pool,
) -> Result<Vec<Row>, AppError> {
    let client = pool.get().await.map_err(|error| {
        tracing::error!("Couldn't get postgres client: {:?}", error);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Server error".to_string(),
        )
    })?;

    let stmt = client.prepare(query).await.map_err(|error| {
        tracing::error!("Couldn't prepare statement: {:?}", error);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Server error".to_string(),
        )
    })?;

    let row = client.query(&stmt, params).await.map_err(|error| {
        tracing::error!("Couldn't query statement: {:?}", error);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Server error".to_string(),
        )
    })?;

    Ok(row)
}

pub async fn query_one(
    query: &str,
    params: &[&(dyn ToSql + Sync)],
    pool: &Pool,
    error_field_name: Option<String>,
) -> Result<Row, AppError> {
    let client = pool.get().await.map_err(|error| {
        tracing::error!("Couldn't get postgres client: {:?}", error);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Server error".to_string(),
        )
    })?;

    let stmt = client.prepare(query).await.map_err(|error| {
        tracing::error!("Couldn't prepare statement: {:?}", error);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Server error".to_string(),
        )
    })?;

    let row = client.query_one(&stmt, params).await.map_err(|error| {
        tracing::error!("Couldn't query statement: {:?}", error);
        match error_field_name {
            Some(field_name) => AppError::new(
                StatusCode::NOT_FOUND,
                format!("{} not found", field_name),
            ),
            None => AppError::new(
                StatusCode::NOT_FOUND,
                "Not found".to_string(),
            ),
        }
    })?;

    Ok(row)
}
