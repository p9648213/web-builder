use axum::http::StatusCode;
use deadpool_postgres::Pool;
use tokio_postgres::{types::ToSql, Row};

use crate::models::error::AppError;

pub async fn query_db(
    query: &str,
    params: &[&(dyn ToSql + Sync)],
    pool: &Pool,
) -> Result<Vec<Row>, AppError> {
    let client = pool.get().await.map_err(|error| {
        tracing::error!("Couldn't get postgres client: {:?}", error);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Couldn't get postgres client".to_string(),
        )
    })?;

    let stmt = client.prepare(query).await.map_err(|error| {
        tracing::error!("Couldn't prepare statement: {:?}", error);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Couldn't prepare statement".to_string(),
        )
    })?;

    let row = client.query(&stmt, params).await.map_err(|error| {
        tracing::error!("Couldn't query statement: {:?}", error);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Couldn't query statement".to_string(),
        )
    })?;

    Ok(row)
}
