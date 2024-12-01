use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
    Extension,
};
use axum_csrf::CsrfToken;
use deadpool_postgres::Pool;
use reqwest::StatusCode;

use crate::{
    middlewares::auth::UserId,
    models::{
        error::AppError, rso_data::RsoData, template::Template, website::Website,
        website_template::WebsiteTemplate,
    },
    views::builder::{
        data::render_setup_data,
        template::{render_choose_template, render_no_website, render_website_template},
        website::render_create_website,
    },
};

pub async fn get_section(
    Path(section): Path<String>,
    token: CsrfToken,
    State(pg_pool): State<Pool>,
    Extension(user_id): Extension<UserId>,
) -> Result<impl IntoResponse, AppError> {
    let authenticity_token = token.authenticity_token().unwrap_or("".to_owned());

    let row = Website::get_website_by_user_id(user_id.0, &pg_pool).await?;

    let website: Option<Website> = if let Some(row) = row {
        Some(Website::try_from(row))
    } else {
        None
    };

    match section.as_str() {
        "template" => {
            if let Some(website) = website {
                if let Some(template_id) = website.template_id {
                    let website_id = website.id.ok_or_else(|| {
                        tracing::error!("No id column or value is null");
                        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
                    })?;

                    let row = WebsiteTemplate::get_website_template_by_template_id_and_website_id(
                        website_id,
                        template_id,
                        &pg_pool,
                    )
                    .await?
                    .ok_or_else(|| {
                        tracing::error!(
                            "The template with id {template_id} is not exit in table templates"
                        );
                        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
                    })?;

                    let website_template = WebsiteTemplate::try_from(row);

                    let template_type = website_template.template_type.ok_or_else(|| {
                        tracing::error!("No template_type column or value is null");
                        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
                    })?;

                    Ok(Html(render_website_template(template_type).into_string()).into_response())
                } else {
                    let row = Template::get_all_templates(&pg_pool).await?;
                    let template = Template::try_from_rows(row);

                    Ok(Html(
                        render_choose_template(website, template, authenticity_token)?
                            .into_string(),
                    )
                    .into_response())
                }
            } else {
                Ok(Html(render_no_website().into_string()).into_response())
            }
        }
        "data" => {
            let row = RsoData::get_rso_data_by_user_id(user_id.0, &pg_pool).await?;

            let rso_data = if let Some(row) = row {
                Some(RsoData::try_from(row))
            } else {
                None
            };

            Ok((
                token,
                Html(render_setup_data(authenticity_token, rso_data).into_string()),
            )
                .into_response())
        }
        "website" => Ok(
            Html(render_create_website(authenticity_token, website)?.into_string()).into_response(),
        ),
        _ => Ok(Html("Not found".to_owned()).into_response()),
    }
}
