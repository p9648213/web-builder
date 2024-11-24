use crate::{
    middlewares::auth::UserId,
    models::{error::AppError, website::Website},
    views::builder::website::render_user_website,
};
use axum::{
    extract::{Extension, State},
    response::Html,
    Form,
};
use deadpool_postgres::Pool;
use reqwest::StatusCode;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CreateWebsiteForm {
    pub name: String,
    pub domain: String,
}

pub async fn create_website(
    State(pg_pool): State<Pool>,
    Extension(user_id): Extension<UserId>,
    Form(create_webiste_form): Form<CreateWebsiteForm>,
) -> Result<Html<String>, AppError> {
    let user_id = user_id.0;

    let new_website = Website::new(
        None,
        Some(create_webiste_form.name),
        Some(create_webiste_form.domain),
        None,
        None,
    );

    let result = Website::insert_website(user_id, &new_website, &pg_pool).await?;

    if result == 0 {
        tracing::error!("There was an error inserting the website. No rows were affected.");
        return Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Server error",
        ));
    }

    Ok(Html(render_user_website(new_website)?.into_string()))
}
