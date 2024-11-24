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

    pub fn try_from(row: Row) -> Self {
        let id: Option<i32> = row.try_get("id").unwrap_or(None);
        let username: Option<String> = row.try_get("username").unwrap_or(None);
        let password: Option<String> = row.try_get("password").unwrap_or(None);
        let email: Option<String> = row.try_get("email").unwrap_or(None);
        let role: Option<Role> = row.try_get("role").unwrap_or(None);

        User {
            id,
            username,
            password,
            email,
            role,
        }
    }

    pub async fn get_user_by_id(id: i32, pool: &Pool) -> Result<Option<Row>, AppError> {
        query_optional("SELECT * FROM users WHERE id = $1", &[&id], pool).await
    }

    pub async fn get_user_by_email(email: &str, pool: &Pool) -> Result<Option<Row>, AppError> {
        query_optional("SELECT * FROM users WHERE email = $1", &[&email], pool).await
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
