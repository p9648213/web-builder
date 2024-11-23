use deadpool_postgres::Pool;
use tokio_postgres::Row;

use crate::utilities::db::{excute, query_optional};

use super::error::{AppError, DtoError};

pub struct Website {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub domain: Option<String>,
}

impl Website {
    pub fn new(id: Option<i32>, name: Option<String>, domain: Option<String>) -> Self {
        Self { id, name, domain }
    }

    fn try_from(row: Row) -> Self {
        let id: Option<i32> = row.try_get("id").unwrap_or(None);
        let name: Option<String> = row.try_get("name").unwrap_or(None);
        let domain: Option<String> = row.try_get("domain").unwrap_or(None);

        Website { id, name, domain }
    }

    pub fn from_row<T: FromWebsite>(row: Row) -> Result<T, DtoError> {
        let website = Website::try_from(row);
        T::from_website(website)
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
            "SELECT * FROM websites WHERE user_id = $1",
            &[&user_id],
            pool,
        )
        .await
    }
}

pub trait FromWebsite: Sized {
    fn from_website(website: Website) -> Result<Self, DtoError>;
}

#[derive(Debug)]
pub struct WebsiteDTO {
    pub name: String,
    pub domain: String,
}

impl FromWebsite for WebsiteDTO {
    fn from_website(website: Website) -> Result<Self, DtoError> {
        Ok(WebsiteDTO {
            name: website
                .name
                .ok_or(DtoError::new("WebsiteDTO convert error: Name not found"))?,
            domain: website
                .domain
                .ok_or(DtoError::new("WebsiteDTO convert error: Domain not found"))?,
        })
    }
}
