use super::error::AppError;
use crate::utilities::db::{excute, query_optional};
use deadpool_postgres::Pool;
use postgres_types::{FromSql, ToSql};
use tokio_postgres::Row;

#[derive(Debug, ToSql, FromSql, Clone)]
pub enum Role {
    User,
    Admin,
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: Option<i32>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub role: Option<Role>,
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
            role: None,
        }
    }

    pub fn try_from(row: &Row, prefix: Option<&str>) -> Self {
        let prefix = prefix.unwrap_or("");

        let id: Option<i32> = row.try_get(format!("{}id", prefix).as_str()).unwrap_or(None);
        let username: Option<String> = row.try_get(format!("{}username", prefix).as_str()).unwrap_or(None);
        let password: Option<String> = row.try_get(format!("{}password", prefix).as_str()).unwrap_or(None);
        let email: Option<String> = row.try_get(format!("{}email", prefix).as_str()).unwrap_or(None);
        let role: Option<Role> = row.try_get(format!("{}role", prefix).as_str()).unwrap_or(None);

        Self {
            id,
            username,
            password,
            email,
            role,
        }
    }

    pub async fn get_user_by_id(
        id: i32,
        pool: &Pool,
        columns: Vec<&str>,
    ) -> Result<Option<Row>, AppError> {
        let columns = columns.join(",");

        query_optional(
            &format!("SELECT {} FROM users WHERE id = $1", columns),
            &[&id],
            pool,
        )
        .await
    }

    pub async fn get_user_by_email(
        email: &str,
        pool: &Pool,
        columns: Vec<&str>,
    ) -> Result<Option<Row>, AppError> {
        let columns = columns.join(",");

        query_optional(
            &format!("SELECT {} FROM users WHERE email = $1", columns),
            &[&email],
            pool,
        )
        .await
    }

    pub async fn insert_user(user: User, pool: &Pool) -> Result<u64, AppError> {
        excute(
            "INSERT INTO users (username, password, email) VALUES ($1, $2, $3)",
            &[&user.username, &user.password, &user.email],
            pool,
        )
        .await
    }
}
