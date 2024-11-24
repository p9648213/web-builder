use deadpool_postgres::Pool;
use postgres_types::{FromSql, ToSql};
use tokio_postgres::Row;

use crate::utilities::db::query;

use super::error::AppError;

#[derive(Debug, ToSql, FromSql, Clone)]
pub enum TemplateType {
    Ecommerce,
    Custom,
}

pub struct Template {
    pub id: Option<i32>,
    pub template_type: Option<TemplateType>,
    pub description: Option<String>,
}

impl Template {
    pub fn new(
        id: Option<i32>,
        template_type: Option<TemplateType>,
        description: Option<String>,
    ) -> Self {
        Self {
            id,
            template_type,
            description,
        }
    }

    pub fn try_from(row: Row) -> Self {
        let id: Option<i32> = row.try_get("id").unwrap_or(None);
        let template_type: Option<TemplateType> = row.try_get("template_type").unwrap_or(None);
        let description: Option<String> = row.try_get("description").unwrap_or(None);

        Template {
            id,
            template_type,
            description,
        }
    }

    pub async fn get_all_templates(pool: &Pool) -> Result<Vec<Row>, AppError> {
        query("SELECT * FROM templates", &[], pool).await
    }
}
