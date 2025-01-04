use axum::{extract::Path, response::Html, Extension};
use maud::html;
use serde::Deserialize;

use crate::{
    middlewares::auth::UserId,
    models::error::AppError,
    views::builder::{
        data::render_setup_data_page, edit::render_edit_style_page,
        template::render_website_template_page, website::render_create_website_page,
    },
};

pub enum BuilderStyle {
    Header,
    Footer,
    Home,
    SearchResult,
    PropertyDetail,
    Contact,
}

pub enum BuilderSection {
    ChooseStyle,
    Branding,
    Content,
    AdvancedSetting,
}

#[derive(Deserialize)]
pub struct EditParams {
    pub website_id: i32,
    pub section: String,
    pub sub_section: String,
}

pub async fn get_create_website_page() -> Html<String> {
    Html(render_create_website_page().into_string())
}

pub async fn get_select_template_page() -> Html<String> {
    Html(render_website_template_page().into_string())
}

pub async fn get_setup_data_page() -> Html<String> {
    Html(render_setup_data_page().into_string())
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
                    (render_edit_style_page(BuilderSection::ChooseStyle ,BuilderStyle::Header))
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
