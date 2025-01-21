use deadpool_postgres::Pool;
use tokio_postgres::Row;

use crate::utilities::db::query_optional;

use super::{error::AppError, website::Website, website_setting::WebsiteSetting};

pub struct WebsiteJoinWebsiteSetting {
    pub website: Website,
    pub website_setting: WebsiteSetting,
}

impl WebsiteJoinWebsiteSetting {
    pub fn try_from(row: &Row, website_prefix: &str, website_settings_prefix: &str) -> Self {
        Self {
            website: Website::try_from(&row, Some(website_prefix)),
            website_setting: WebsiteSetting::try_from(&row, Some(website_settings_prefix)),
        }
    }

    pub async fn get_website_setting_by_domain(
        domain: &str,
        pool: &Pool,
        website_columns: Option<Vec<&str>>,
        website_settings_columns: Option<Vec<&str>>,
        website_prefix: Option<&str>,
        website_settings_prefix: Option<&str>,
    ) -> Result<Option<Row>, AppError> {
        let mut website_query_columns = vec![];
        let mut website_settings_query_columns = vec![];

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

        if website_settings_columns.is_some() && website_settings_prefix.is_some() {
            let website_settings_columns = website_settings_columns.unwrap();
            let website_settings_prefix = website_settings_prefix.unwrap();

            let column_with_prefix: Vec<String> = website_settings_columns
                .iter()
                .map(|value| {
                    format!(
                        "{}.{} as {}_{}",
                        website_settings_prefix, value, website_settings_prefix, value
                    )
                })
                .collect();

            website_settings_query_columns = column_with_prefix;
        }

        let website_prefix = website_prefix.unwrap_or("websites");
        let website_settings_prefix = website_settings_prefix.unwrap_or("website_settings");

        website_query_columns.extend(website_settings_query_columns);

        query_optional(
            &format!(
                "SELECT {} FROM websites {} JOIN website_settings {}
                ON {}.website_id = {}.id WHERE {}.domain = $1",
                website_query_columns.join(","),
                website_prefix,
                website_settings_prefix,
                website_settings_prefix,
                website_prefix,
                website_prefix
            ),
            &[&domain],
            pool,
        )
        .await
    }
}
