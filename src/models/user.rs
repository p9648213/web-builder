use deadpool_postgres::Pool;
use tokio_postgres::Row;

use crate::utils::db::query_db;

use super::error::AppError;

#[derive(Debug)]
pub struct User {
    pub id: Option<i32>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
}

impl User {
    pub fn new(
        id: Option<i32>,
        username: Option<String>,
        password: Option<String>,
        email: Option<String>,
    ) -> Self {
        Self {
            id,
            username,
            password,
            email,
        }
    }

    pub fn from_row(row: Row) -> Result<Self, AppError> {
        let id: Option<i32> = row.try_get("id").unwrap_or(None);
        let username: Option<String> = row.try_get("username").unwrap_or(None);
        let password: Option<String> = row.try_get("password").unwrap_or(None);
        let email: Option<String> = row.try_get("email").unwrap_or(None);

        Ok(Self {
            id,
            username,
            password,
            email,
        })
    }

    pub async fn get_all_users(pool: &Pool) -> Result<Vec<Row>, AppError> {
        let result = query_db("SELECT id FROM users", &[], pool).await?;
        Ok(result)
    }
}
