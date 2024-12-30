use axum::{extract::{Path, State}, response::Html, Extension};
use deadpool_postgres::Pool;
use maud::html;
use serde::Deserialize;

use crate::{middlewares::auth::UserId, models::error::AppError, views::builder::edit::render_edit_page};

#[derive(Deserialize)]
pub struct EditParams {
    pub website_id: i32,
}

pub async fn get_edit_page(
  Path(EditParams { website_id }): Path<EditParams>,
  State(pg_pool): State<Pool>,
  Extension(user_id): Extension<UserId>,
) -> Result<Html<String>, AppError> {
    let html = html! {
      (render_edit_page())
      div { (website_id) }
      div { (user_id.0) }
    }
    .into_string();

    Ok(Html(html))
}
