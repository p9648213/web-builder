use deadpool_postgres::Pool;
use tokio_postgres::Row;

use crate::utilities::db::query_optional;

use super::{
    error::AppError,
    template::{Template, TemplateType},
    website::Website,
};

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
        columns: Vec<&str>,
    ) -> Result<Option<Row>, AppError> {
        let columns = columns.join(",");

        query_optional(
          &format!("SELECT {} FROM websites JOIN templates ON websites.template_id = templates.id WHERE websites.id = $1 AND templates.id = $2", columns),
          &[&website_id, &template_id],
          pool,
        )
        .await
    }
}

pub struct WebsiteJoinTemplate {
    pub website: Website,
    pub template: Template,
}

impl WebsiteJoinTemplate {
    pub fn try_from(
        row: &Row,
        website_prefix: &str,
        template_prefix: &str,
    ) -> Result<Self, AppError> {
        Ok(Self {
            website: Website::try_from(&row, Some(website_prefix)),
            template: Template::try_from(&row, Some(template_prefix)),
        })
    }

    pub async fn get_website_template_by_template_id_and_website_id(
        website_id: i32,
        template_id: i32,
        pool: &Pool,
        website_columns: Option<Vec<&str>>,
        template_columns: Option<Vec<&str>>,
        website_prefix: Option<&str>,
        template_prefix: Option<&str>,
    ) -> Result<Option<Row>, AppError> {
        let mut website_query_columns: Option<Vec<String>> = None;
        let mut template_query_columns: Option<Vec<String>> = None;

        if website_columns.is_some() && website_prefix.is_some() {
            let website_columns = website_columns.unwrap();
            let website_prefix = website_prefix.unwrap();

            let column_with_prefix: Vec<String> = website_columns
                .iter()
                .map(|value| {
                    format!(
                        "{}.{} as {}_{}",
                        website_prefix, value, website_prefix, value
                    )
                })
                .collect();

            website_query_columns = Some(column_with_prefix);
        }

        if template_columns.is_some() && template_prefix.is_some() {
            let template_columns = template_columns.unwrap();
            let template_prefix = template_prefix.unwrap();

            let column_with_prefix: Vec<String> = template_columns
                .iter()
                .map(|value| {
                    format!(
                        "{}.{} as {}_{}",
                        template_prefix, value, template_prefix, value
                    )
                })
                .collect();

            template_query_columns = Some(column_with_prefix);
        }

        let website_prefix = website_prefix.unwrap_or("websites");
        let template_prefix = template_prefix.unwrap_or("templates");

        let select_columns = format!("{}{}", website_query_columns.unwrap_or(vec![]).join(","), template_query_columns.unwrap_or(vec![]));

        query_optional(
          &format!("SELECT {},{} FROM websites {} JOIN templates {} ON {}.template_id = {}.id WHERE {}.id = $1 AND {}.id = $2", website_query_columns.join(","), template_query_columns.join(","), &website_prefix, &template_prefix, &website_prefix, &template_prefix, &website_prefix, &template_prefix),
          &[&website_id, &template_id],
          pool,
        )
        .await
    }
}
