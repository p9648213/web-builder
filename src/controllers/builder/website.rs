use crate::{
    middlewares::auth::UserId,
    models::{
        error::AppError, template::Template, website::Website, website_setting::WebsiteSetting,
    },
    views::builder::{template::render_website_template, website::render_user_website},
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

#[derive(Deserialize, Debug)]
pub struct SelectTemplateForm {
    pub website_id: i32,
    pub template_id: i32,
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

    let row = Website::get_website_by_domain_name(&new_website.domain.as_ref().unwrap(), &pg_pool)
        .await?;

    if let Some(row) = row {
        let created_website = Website::try_from(row);

        let website_id = created_website.id.ok_or_else(|| {
            tracing::error!("No id column or value is null");
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        let new_website_setting = WebsiteSetting::new(
            None,
            Some(website_id),
            Some(1),
            Some(1),
            Some(1),
            Some(1),
            Some(1),
            Some(1),
        );

        let result =
            WebsiteSetting::insert_website_setting(website_id, &new_website_setting, &pg_pool)
                .await?;

        if result == 0 {
            tracing::error!(
                "There was an error inserting the website setting. No rows were affected."
            );
            return Err(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Server error",
            ));
        }

        Ok(Html(render_user_website(&created_website)?.into_string()))
    } else {
        tracing::error!("There was an error getting the website by domain name.");
        Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Server error",
        ))
    }
}

pub async fn select_template_for_webiste(
    State(pg_pool): State<Pool>,
    Form(select_template_form): Form<SelectTemplateForm>,
) -> Result<Html<String>, AppError> {
    let website_id = select_template_form.website_id;
    let template_id = select_template_form.template_id;

    let result = Website::update_template_by_website_id(website_id, template_id, &pg_pool).await?;

    if result == 0 {
        tracing::error!("There was an error update template for website. No rows were affected.");
        return Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Server error",
        ));
    }

    let row = Template::get_template_by_id(template_id, &pg_pool)
        .await?
        .ok_or_else(|| {
            tracing::error!("Can not find template with template_id: {template_id}");
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

    let template = Template::try_from(row);

    let template_type = template.template_type.ok_or_else(|| {
        tracing::error!("No template_type column or value is null");
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
    })?;

    Ok(Html(render_website_template(template_type).into_string()))
}
