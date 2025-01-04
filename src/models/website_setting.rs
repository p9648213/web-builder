use tokio_postgres::Row;

use crate::utilities::db::{excute, query_optional};

use super::error::AppError;

pub struct WebsiteSetting {
    pub id: Option<i32>,
    pub website_id: Option<i32>,
    pub header_theme: Option<i32>,
    pub footer_theme: Option<i32>,
    pub home_theme: Option<i32>,
    pub search_theme: Option<i32>,
    pub property_theme: Option<i32>,
    pub contact_theme: Option<i32>,
}

impl WebsiteSetting {
    pub fn new(
        id: Option<i32>,
        website_id: Option<i32>,
        header_theme: Option<i32>,
        footer_theme: Option<i32>,
        home_theme: Option<i32>,
        search_theme: Option<i32>,
        property_theme: Option<i32>,
        contact_theme: Option<i32>,
    ) -> Self {
        Self {
            id,
            website_id,
            header_theme,
            footer_theme,
            home_theme,
            search_theme,
            property_theme,
            contact_theme,
        }
    }

    pub fn try_from(row: Row) -> Self {
        let id: Option<i32> = row.try_get("id").unwrap_or(None);
        let website_id: Option<i32> = row.try_get("webiste_id").unwrap_or(None);
        let header_theme: Option<i32> = row.try_get("header_theme").unwrap_or(None);
        let footer_theme: Option<i32> = row.try_get("footer_theme").unwrap_or(None);
        let home_theme: Option<i32> = row.try_get("home_theme").unwrap_or(None);
        let search_theme: Option<i32> = row.try_get("search_theme").unwrap_or(None);
        let property_theme: Option<i32> = row.try_get("property_theme").unwrap_or(None);
        let contact_theme: Option<i32> = row.try_get("contact_theme").unwrap_or(None);

        Self {
            id,
            website_id,
            header_theme,
            footer_theme,
            home_theme,
            search_theme,
            property_theme,
            contact_theme,
        }
    }

    pub async fn insert_website_setting(
        website_id: i32,
        setting: &WebsiteSetting,
        pool: &deadpool_postgres::Pool,
    ) -> Result<u64, AppError> {
        excute(
            "INSERT INTO website_settings (website_id, header_theme, footer_theme, home_theme, search_theme, property_theme, contact_theme) VALUES ($1, $2, $3, $4, $5, $6, $7)",
            &[
                &website_id,
                &setting.header_theme,
                &setting.footer_theme,
                &setting.home_theme,
                &setting.search_theme,
                &setting.property_theme,
                &setting.contact_theme,
            ],
            pool,
        )
        .await
    }

    pub async fn get_website_setting_by_website_id(
        website_id: i32,
        pool: &deadpool_postgres::Pool,
        columns: Vec<&str>,
    ) -> Result<Option<Row>, AppError> {
        let columns = columns.join(",");

        query_optional(
            &format!(
                "SELECT {} FROM website_settings WHERE website_id = $1",
                columns
            ),
            &[&website_id],
            pool,
        )
        .await
    }

    pub async fn update_header_theme_by_website_id(
        website_id: i32,
        header_theme: i32,
        pool: &deadpool_postgres::Pool,
    ) -> Result<u64, AppError> {
        excute(
            "UPDATE website_settings SET header_theme = $1 WHERE webiste_id = $2",
            &[&header_theme, &website_id],
            pool,
        )
        .await
    }
}
