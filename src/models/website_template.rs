use deadpool_postgres::Pool;
use tokio_postgres::Row;

use crate::utilities::db::query_optional;

use super::{error::AppError, template::TemplateType};

pub struct WebsiteTemplate {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub domain: Option<String>,
    pub template_id: Option<i32>,
    pub user_id: Option<i32>,
    pub template_type: Option<TemplateType>,
    pub description: Option<String>,
}

impl WebsiteTemplate {
    pub fn new(
        id: Option<i32>,
        name: Option<String>,
        domain: Option<String>,
        template_id: Option<i32>,
        user_id: Option<i32>,
        template_type: Option<TemplateType>,
        description: Option<String>,
    ) -> Self {
        Self {
            id,
            name,
            domain,
            template_id,
            user_id,
            template_type,
            description,
        }
    }

    pub fn try_from(row: Row) -> Self {
        let id: Option<i32> = row.try_get("id").unwrap_or(None);
        let name: Option<String> = row.try_get("name").unwrap_or(None);
        let domain: Option<String> = row.try_get("domain").unwrap_or(None);
        let template_id: Option<i32> = row.try_get("template_id").unwrap_or(None);
        let user_id: Option<i32> = row.try_get("user_id").unwrap_or(None);
        let template_type: Option<TemplateType> = row.try_get("template_type").unwrap_or(None);
        let description: Option<String> = row.try_get("description").unwrap_or(None);

        Self {
            id,
            name,
            domain,
            template_id,
            user_id,
            template_type,
            description,
        }
    }

    pub async fn get_website_template_by_template_id_and_website_id(
        website_id: i32,
        template_id: i32,
        pool: &Pool,
    ) -> Result<Option<Row>, AppError> {
        query_optional(
          "SELECT template_type FROM websites JOIN templates ON websites.template_id = $1 WHERE websites.id = $2",
          &[&template_id, &website_id],
          pool,
      )
      .await
    }
}
