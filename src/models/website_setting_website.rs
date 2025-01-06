use deadpool_postgres::Pool;
use tokio_postgres::Row;

use crate::utilities::db::query_optional;

use super::error::AppError;

#[derive(Debug)]
pub struct WebsiteSettingWebsite {
    pub id: Option<i32>,
    pub website_id: Option<i32>,
    pub user_id: Option<i32>,
    pub header_theme: Option<i32>,
    pub footer_theme: Option<i32>,
    pub home_theme: Option<i32>,
    pub search_theme: Option<i32>,
    pub property_theme: Option<i32>,
    pub contact_theme: Option<i32>,
    pub name: Option<String>,
    pub domain: Option<String>,
    pub template_id: Option<i32>,
}

impl WebsiteSettingWebsite {
    pub fn new(
        id: Option<i32>,
        website_id: Option<i32>,
        user_id: Option<i32>,
        header_theme: Option<i32>,
        footer_theme: Option<i32>,
        home_theme: Option<i32>,
        search_theme: Option<i32>,
        property_theme: Option<i32>,
        contact_theme: Option<i32>,
        name: Option<String>,
        domain: Option<String>,
        template_id: Option<i32>,
    ) -> Self {
        Self {
            id,
            website_id,
            user_id,
            header_theme,
            footer_theme,
            home_theme,
            search_theme,
            property_theme,
            contact_theme,
            name,
            domain,
            template_id,
        }
    }

    pub fn try_from(row: tokio_postgres::Row) -> Self {
        let id: Option<i32> = row.try_get("id").unwrap_or(None);
        let website_id: Option<i32> = row.try_get("website_id").unwrap_or(None);
        let user_id: Option<i32> = row.try_get("user_id").unwrap_or(None);
        let header_theme: Option<i32> = row.try_get("header_theme").unwrap_or(None);
        let footer_theme: Option<i32> = row.try_get("footer_theme").unwrap_or(None);
        let home_theme: Option<i32> = row.try_get("home_theme").unwrap_or(None);
        let search_theme: Option<i32> = row.try_get("search_theme").unwrap_or(None);
        let property_theme: Option<i32> = row.try_get("property_theme").unwrap_or(None);
        let contact_theme: Option<i32> = row.try_get("contact_theme").unwrap_or(None);
        let name: Option<String> = row.try_get("name").unwrap_or(None);
        let domain: Option<String> = row.try_get("domain").unwrap_or(None);
        let template_id: Option<i32> = row.try_get("template_id").unwrap_or(None);

        Self {
            id,
            website_id,
            user_id,
            header_theme,
            footer_theme,
            home_theme,
            search_theme,
            property_theme,
            contact_theme,
            name,
            domain,
            template_id,
        }
    }

    pub async fn get_website_setting_by_domain(
        domain: &str,
        pool: &Pool,
        columns: Vec<&str>,
    ) -> Result<Option<Row>, AppError> {
        let columns = columns.join(",");

        query_optional(
            &format!(
                "SELECT {} FROM websites JOIN website_settings 
                ON website_settings.website_id = websites.id WHERE domain = $1",
                columns,
            ),
            &[&domain],
            pool,
        )
        .await
    }
}
