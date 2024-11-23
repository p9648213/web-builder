use crate::{
    middlewares::auth::UserId,
    models::{
        error::AppError,
        website::{Website, WebsiteDTO},
    },
    views::builder::website::render_user_website,
};
use axum::{
    extract::{Extension, State},
    response::Html,
    Form,
};
use deadpool_postgres::Pool;
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
    );

    Website::insert_website(user_id, &new_website, &pg_pool).await?;

    Ok(Html(
        render_user_website(WebsiteDTO {
            name: new_website.name.unwrap(),
            domain: new_website.domain.unwrap(),
        })
        .into_string(),
    ))
}
