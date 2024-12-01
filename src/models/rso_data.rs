use deadpool_postgres::Pool;
use tokio_postgres::Row;

use crate::utilities::db::{excute, query_optional};

use super::error::AppError;

pub struct RsoData {
    pub id: Option<i32>,
    pub user_id: Option<i32>,
    pub identifier_id: Option<i32>,
    pub api_key: Option<String>,
    pub filter_id_sale: Option<i32>,
    pub filter_id_long: Option<i32>,
    pub filter_id_short: Option<i32>,
    pub filter_id_featured: Option<i32>,
}

impl RsoData {
    pub fn new(
        id: Option<i32>,
        user_id: Option<i32>,
        identifier_id: Option<i32>,
        api_key: Option<String>,
        filter_id_sale: Option<i32>,
        filter_id_long: Option<i32>,
        filter_id_short: Option<i32>,
        filter_id_featured: Option<i32>,
    ) -> Self {
        Self {
            id,
            user_id,
            identifier_id,
            api_key,
            filter_id_sale,
            filter_id_long,
            filter_id_short,
            filter_id_featured,
        }
    }

    pub fn try_from(row: Row) -> Self {
        let id: Option<i32> = row.try_get("id").unwrap_or(None);
        let user_id: Option<i32> = row.try_get("user_id").unwrap_or(None);
        let identifier_id: Option<i32> = row.try_get("identifier_id").unwrap_or(None);
        let api_key: Option<String> = row.try_get("api_key").unwrap_or(None);
        let filter_id_sale: Option<i32> = row.try_get("filter_id_sale").unwrap_or(None);
        let filter_id_long: Option<i32> = row.try_get("filter_id_long").unwrap_or(None);
        let filter_id_short: Option<i32> = row.try_get("filter_id_short").unwrap_or(None);
        let filter_id_featured: Option<i32> = row.try_get("filter_id_featured").unwrap_or(None);

        Self {
            id,
            user_id,
            identifier_id,
            api_key,
            filter_id_sale,
            filter_id_long,
            filter_id_short,
            filter_id_featured,
        }
    }

    pub async fn create_rso_data_by_user_id(
        user_id: i32,
        rso_data: &RsoData,
        pool: &Pool,
    ) -> Result<u64, AppError> {
        excute(
            "INSERT INTO rso_data (user_id, identifier_id, api_key, filter_id_sale, filter_id_long, filter_id_short, filter_id_featured) VALUES ($1, $2, $3, $4, $5, $6, $7)",
            &[
                &user_id,
                &rso_data.identifier_id,
                &rso_data.api_key,
                &rso_data.filter_id_sale,
                &rso_data.filter_id_long,
                &rso_data.filter_id_short,
                &rso_data.filter_id_featured,
            ],
            pool,
        )
        .await
    }

    pub async fn update_rso_data_by_user_id(
        user_id: i32,
        rso_data: &RsoData,
        pool: &Pool,
    ) -> Result<u64, AppError> {
        excute(
            "UPDATE rso_data SET identifier_id = $1, api_key = $2, filter_id_sale = $3, filter_id_long = $4, filter_id_short = $5, filter_id_featured = $6 WHERE user_id = $7",
            &[
                &rso_data.identifier_id,
                &rso_data.api_key,
                &rso_data.filter_id_sale,
                &rso_data.filter_id_long,
                &rso_data.filter_id_short,
                &rso_data.filter_id_featured,
                &user_id,
            ],
            pool,
        )
        .await
    }

    pub async fn update_status_rso_data_by_user_id(
        user_id: i32,
        status: bool,
        pool: &Pool,
    ) -> Result<u64, AppError> {
        excute(
            "UPDATE rso_data SET status = $1 WHERE user_id = $2",
            &[&status, &user_id],
            pool,
        )
        .await
    }

    pub async fn get_rso_data_by_user_id(
        user_id: i32,
        pool: &Pool,
    ) -> Result<Option<Row>, AppError> {
        query_optional(
            "SELECT * FROM rso_data WHERE user_id = $1",
            &[&user_id],
            pool,
        )
        .await
    }
}
