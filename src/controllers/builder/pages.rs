use axum::{extract::Path, response::Html, Extension};
use maud::html;
use serde::Deserialize;

use crate::{
    middlewares::auth::UserId,
    models::error::AppError,
    views::builder::{edit::render_edit_header_page, home::render_setup_page},
};

#[derive(Deserialize)]
pub struct EditParams {
    pub website_id: i32,
    pub section: String,
    pub sub_section: String,
}

pub async fn get_builder_setup_page() -> Html<String> {
    Html(render_setup_page().into_string())
}

pub async fn get_edit_page(
    Path(EditParams {
        website_id,
        section,
        sub_section,
    }): Path<EditParams>,
    Extension(user_id): Extension<UserId>,
) -> Result<Html<String>, AppError> {
    let html = match section.as_str() {
        "style" => {
            match sub_section.as_str() {
                "header" => html! {
                    (render_edit_header_page())
                },
                _ => html! {
                    "Not Found"
                },
            }
        }
        .into_string(),
        _ => html! {
            "Not Found"
        }
        .into_string(),
    };

    Ok(Html(html))
}
