use deadpool_postgres::Pool;
use tokio_postgres::Row;
use crate::utilities::db::query_one;
use super::error::AppError;

#[derive(Debug)]
pub struct User {
    pub id: Option<i64>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub role: Option<String>,
}

impl User {
    pub fn new(
        id: Option<i64>,
        username: Option<String>,
        password: Option<String>,
        email: Option<String>,
    ) -> Self {
        Self {
            id,
            username,
            password,
            email,
            role: None
        }
    }

    pub async fn get_user_by_id(id: i64, pool: &Pool) -> Result<Row, AppError> {
        query_one("SELECT * FROM users WHERE id = $1", &[&id], pool, Some("Id".to_string())).await
    }

    pub async fn get_user_by_email(email: String, pool: &Pool) -> Result<Row, AppError> {
        query_one("SELECT * FROM users WHERE email = $1", &[&email], pool, Some("Email".to_string())).await
    }

    pub fn from_row<T: FromUser>(row: Row) -> Result<T, String> {
        let user = User::try_from(row);
        T::from_user(user)
    }
}

impl User {
    fn try_from(row: Row) -> Self {
        let id: Option<i64> = row.try_get("id").unwrap_or(None);
        let username: Option<String> = row.try_get("username").unwrap_or(None);
        let password: Option<String> = row.try_get("password").unwrap_or(None);
        let email: Option<String> = row.try_get("email").unwrap_or(None);
        let role: Option<String> = row.try_get("role").unwrap_or(None);

        User {
            id,
            username,
            password,
            email,
            role,
        }
    }
}

pub trait FromUser: Sized {
    fn from_user(user: User) -> Result<Self, String>;
}

pub struct UserDTO {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub email: String,
    pub role: String,
}

impl FromUser for UserDTO {
    fn from_user(user: User) -> Result<Self, String> {
        Ok(UserDTO {
            id: user.id.ok_or("UserDTO convert error: Id not found".to_string())?,
            password: user.password.ok_or("UserDTO convert error: Password not found".to_string())?,
            username: user.username.ok_or("UserDTO convert error: Username not found".to_string())?,
            email: user.email.ok_or("UserDTO convert error: Email not found".to_string())?,
            role: user.role.ok_or("UserDTO convert error: Role not found".to_string())?,
        })
    }
}