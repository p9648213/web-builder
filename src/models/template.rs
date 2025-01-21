use deadpool_postgres::Pool;
use postgres_types::{FromSql, ToSql};
use tokio_postgres::Row;

use crate::utilities::db::{query, query_optional};

use super::error::AppError;

#[derive(Debug, ToSql, FromSql, Clone)]
pub enum TemplateType {
    RealEstate,
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

    pub fn try_from(row: &Row, prefix: Option<&str>) -> Self {
        let prefix = prefix.unwrap_or("");

        let id: Option<i32> = row
            .try_get(format!("{}id", prefix).as_str())
            .unwrap_or(None);
        let template_type: Option<TemplateType> = row
            .try_get(format!("{}template_type", prefix).as_str())
            .unwrap_or(None);
        let description: Option<String> = row
            .try_get(format!("{}description", prefix).as_str())
            .unwrap_or(None);

        Self {
            id,
            template_type,
            description,
        }
    }

    pub fn try_from_rows(vec_row: Vec<Row>) -> Vec<Self> {
        let mut templates: Vec<Self> = vec![];

        for row in vec_row {
            let id: Option<i32> = row.try_get("id").unwrap_or(None);
            let template_type: Option<TemplateType> = row.try_get("template_type").unwrap_or(None);
            let description: Option<String> = row.try_get("description").unwrap_or(None);

            templates.push(Template {
                id,
                template_type,
                description,
            });
        }

        templates
    }

    pub async fn get_all_templates(pool: &Pool, columns: Vec<&str>) -> Result<Vec<Row>, AppError> {
        let columns = columns.join(",");
        query(&format!("SELECT {} FROM templates", columns), &[], pool).await
    }

    pub async fn get_template_by_id(
        template_id: i32,
        pool: &Pool,
        columns: Vec<&str>,
    ) -> Result<Option<Row>, AppError> {
        let columns = columns.join(",");

        query_optional(
            &format!("SELECT {} FROM templates WHERE templates.id = $1", columns),
            &[&template_id],
            pool,
        )
        .await
    }
}
