use tokio_postgres::Row;

use crate::utilities::db::{excute, query_optional};

use super::error::AppError;

pub struct WebsiteSetting {
    pub id: Option<i32>,
    pub website_id: Option<i32>,
    pub user_id: Option<i32>,
    pub header_theme: Option<i32>,
    pub footer_theme: Option<i32>,
    pub home_theme: Option<i32>,
    pub search_theme: Option<i32>,
    pub property_theme: Option<i32>,
    pub contact_theme: Option<i32>,
}

impl WebsiteSetting {
    pub fn try_from(row: &Row, prefix: Option<&str>) -> Self {
        let prefix = prefix.unwrap_or("");

        let id: Option<i32> = row
            .try_get(format!("{}id", prefix).as_str())
            .unwrap_or(None);
        let website_id: Option<i32> = row
            .try_get(format!("{}webiste_id", prefix).as_str())
            .unwrap_or(None);
        let user_id: Option<i32> = row
            .try_get(format!("{}user_id", prefix).as_str())
            .unwrap_or(None);
        let header_theme: Option<i32> = row
            .try_get(format!("{}header_theme", prefix).as_str())
            .unwrap_or(None);
        let footer_theme: Option<i32> = row
            .try_get(format!("{}footer_theme", prefix).as_str())
            .unwrap_or(None);
        let home_theme: Option<i32> = row
            .try_get(format!("{}home_theme", prefix).as_str())
            .unwrap_or(None);
        let search_theme: Option<i32> = row
            .try_get(format!("{}search_theme", prefix).as_str())
            .unwrap_or(None);
        let property_theme: Option<i32> = row
            .try_get(format!("{}property_theme", prefix).as_str())
            .unwrap_or(None);
        let contact_theme: Option<i32> = row
            .try_get(format!("{}contact_theme", prefix).as_str())
            .unwrap_or(None);

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
        }
    }

    pub async fn insert_website_setting(
        website_id: i32,
        setting: &WebsiteSetting,
        pool: &deadpool_postgres::Pool,
    ) -> Result<u64, AppError> {
        excute(
            "INSERT INTO website_settings (website_id, header_theme, footer_theme, home_theme, search_theme, property_theme, contact_theme, user_id) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
            &[
                &website_id,
                &setting.header_theme,
                &setting.footer_theme,
                &setting.home_theme,
                &setting.search_theme,
                &setting.property_theme,
                &setting.contact_theme,
                &setting.user_id,
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

    pub async fn get_website_setting_by_id_and_user_id(
        id: i32,
        user_id: i32,
        pool: &deadpool_postgres::Pool,
        columns: Vec<&str>,
    ) -> Result<Option<Row>, AppError> {
        let columns = columns.join(",");

        query_optional(
            &format!(
                "SELECT {} FROM website_settings WHERE id = $1 AND user_id = $2",
                columns
            ),
            &[&id, &user_id],
            pool,
        )
        .await
    }

    pub async fn update_header_theme_by_id(
        id: i32,
        header_theme: i32,
        pool: &deadpool_postgres::Pool,
    ) -> Result<u64, AppError> {
        excute(
            "UPDATE website_settings SET header_theme = $1 WHERE id = $2",
            &[&header_theme, &id],
            pool,
        )
        .await
    }

    pub async fn update_footer_theme_by_id(
        id: i32,
        footer_theme: i32,
        pool: &deadpool_postgres::Pool,
    ) -> Result<u64, AppError> {
        excute(
            "UPDATE website_settings SET footer_theme = $1 WHERE id = $2",
            &[&footer_theme, &id],
            pool,
        )
        .await
    }

    pub async fn update_home_theme_by_id(
        id: i32,
        home_theme: i32,
        pool: &deadpool_postgres::Pool,
    ) -> Result<u64, AppError> {
        excute(
            "UPDATE website_settings SET home_theme = $1 WHERE id = $2",
            &[&home_theme, &id],
            pool,
        )
        .await
    }

    pub async fn update_search_theme_by_id(
        id: i32,
        search_theme: i32,
        pool: &deadpool_postgres::Pool,
    ) -> Result<u64, AppError> {
        excute(
            "UPDATE website_settings SET search_theme = $1 WHERE id = $2",
            &[&search_theme, &id],
            pool,
        )
        .await
    }

    pub async fn update_property_theme_by_id(
        id: i32,
        property_theme: i32,
        pool: &deadpool_postgres::Pool,
    ) -> Result<u64, AppError> {
        excute(
            "UPDATE website_settings SET property_theme = $1 WHERE id = $2",
            &[&property_theme, &id],
            pool,
        )
        .await
    }

    pub async fn update_contact_theme_by_id(
        id: i32,
        contact_theme: i32,
        pool: &deadpool_postgres::Pool,
    ) -> Result<u64, AppError> {
        excute(
            "UPDATE website_settings SET contact_theme = $1 WHERE id = $2",
            &[&contact_theme, &id],
            pool,
        )
        .await
    }
}
