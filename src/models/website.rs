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

    pub fn try_from(row: &Row, prefix: Option<&str>) -> Self {
        let prefix = prefix.unwrap_or("");

        let id: Option<i32> = row.try_get(format!("{}id", prefix).as_str()).unwrap_or(None);
        let name: Option<String> = row.try_get(format!("{}name", prefix).as_str()).unwrap_or(None);
        let domain: Option<String> = row.try_get(format!("{}domain", prefix).as_str()).unwrap_or(None);
        let template_id: Option<i32> = row.try_get(format!("{}template_id", prefix).as_str()).unwrap_or(None);
        let user_id: Option<i32> = row.try_get(format!("{}user_id", prefix).as_str()).unwrap_or(None);

        Self {
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
        columns: Vec<&str>,
    ) -> Result<Option<Row>, AppError> {
        let columns = columns.join(",");

        query_optional(
            &format!("SELECT {} FROM websites WHERE user_id = $1", columns),
            &[&user_id],
            pool,
        )
        .await
    }

    pub async fn get_website_by_user_id_and_website_id(
        user_id: i32,
        website_id: i32,
        pool: &Pool,
        columns: Vec<&str>,
    ) -> Result<Option<Row>, AppError> {
        let columns = columns.join(",");

        query_optional(
            &format!(
                "SELECT {} FROM websites WHERE user_id = $1 AND id = $2",
                columns
            ),
            &[&user_id, &website_id],
            pool,
        )
        .await
    }

    pub async fn get_website_by_domain_name(
        domain_name: &str,
        pool: &Pool,
        columns: Vec<&str>,
    ) -> Result<Option<Row>, AppError> {
        let columns = columns.join(",");

        query_optional(
            &format!("SELECT {} FROM websites WHERE domain = $1", columns),
            &[&domain_name],
            pool,
        )
        .await
    }

    pub async fn update_template_by_website_id(
        website_id: i32,
        template_id: i32,
        pool: &Pool,
    ) -> Result<u64, AppError> {
        excute(
            "UPDATE websites SET template_id = $1 where id = $2",
            &[&template_id, &website_id],
            pool,
        )
        .await
    }
}
