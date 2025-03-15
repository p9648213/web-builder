use deadpool_postgres::Pool;
use tokio_postgres::Row;

use crate::utilities::db::query_optional;

use super::{error::AppError, template::Template, website::Website};

pub struct WebsiteJoinTemplate {
    pub website: Website,
    pub template: Template,
}

impl WebsiteJoinTemplate {
    pub fn try_from(row: &Row, website_prefix: &str, template_prefix: &str) -> Self {
        Self {
            website: Website::try_from(row, Some(website_prefix)),
            template: Template::try_from(row, Some(template_prefix)),
        }
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
        let mut website_query_columns = vec![];
        let mut template_query_columns = vec![];

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

            website_query_columns = column_with_prefix;
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

            template_query_columns = column_with_prefix;
        }

        let website_prefix = website_prefix.unwrap_or("websites");
        let template_prefix = template_prefix.unwrap_or("templates");

        website_query_columns.extend(template_query_columns);

        query_optional(
          &format!("SELECT {} FROM websites {} JOIN templates {} ON {}.template_id = {}.id WHERE {}.id = $1 AND {}.id = $2", website_query_columns.join(","), website_prefix, template_prefix, website_prefix, template_prefix, website_prefix,template_prefix),
          &[&website_id, &template_id],
          pool,
        )
        .await
    }
}
