use deadpool_postgres::Pool;
use tokio_postgres::Row;

use crate::utilities::db::{excute, query_optional};

use super::error::AppError;

pub struct Website {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub domain: Option<String>,
    pub template_id: Option<i32>,
    pub user_id: Option<i32>,
}

impl Website {
    pub fn new(
        id: Option<i32>,
        name: Option<String>,
        domain: Option<String>,
        template_id: Option<i32>,
        user_id: Option<i32>,
    ) -> Self {
        Self {
            id,
            name,
            domain,
            template_id,
            user_id,
        }
    }

    pub fn try_from(row: Row) -> Self {
        let id: Option<i32> = row.try_get("id").unwrap_or(None);
        let name: Option<String> = row.try_get("name").unwrap_or(None);
        let domain: Option<String> = row.try_get("domain").unwrap_or(None);
        let template_id: Option<i32> = row.try_get("template_id").unwrap_or(None);
        let user_id: Option<i32> = row.try_get("user_id").unwrap_or(None);

        Website {
            id,
            name,
            domain,
            template_id,
            user_id,
        }
    }

    pub async fn insert_website(
        user_id: i32,
        website: &Website,
        pool: &Pool,
    ) -> Result<u64, AppError> {
        excute(
            "INSERT INTO websites (user_id, name, domain) VALUES ($1, $2, $3)",
            &[&user_id, &website.name, &website.domain],
            pool,
        )
        .await
    }

    pub async fn get_website_by_user_id(
        user_id: i32,
        pool: &Pool,
    ) -> Result<Option<Row>, AppError> {
        query_optional(
            "SELECT id, name, domain FROM websites WHERE user_id = $1",
            &[&user_id],
            pool,
        )
        .await
    }
}
