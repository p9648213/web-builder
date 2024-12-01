use maud::Markup;
use reqwest::StatusCode;

use crate::{
    models::{error::AppError, website::Website},
    views::builder::home::{render_sub_nav, SUB_NAV},
};

pub fn render_create_website(
    authenticity_token: String,
    website: Option<Website>,
) -> Result<Markup, AppError> {
    Ok(maud::html! {
        section id="create-website" {
            @if let Some(website) = website {
                (render_user_website(website)?)
            } @else {
                form
                    hx-post="/builder/website/create"
                    hx-target="#create-website"
                    hx-swap="outerHTML"
                    class="flex flex-col gap-2 max-w-72"
                {
                    input type="hidden" name="authenticity_token" value=(authenticity_token);
                    label {
                    "Website name: "
                    }
                    input type="text" name="name" {}
                    label {
                    "Domain: "
                    }
                    input type="text" name="domain" {}
                    button class="border-gray-500 mt-3 border" {
                    "Create"
                    }
                }
            }
          }
        (render_sub_nav(SUB_NAV, "Create website", Some("outerHTML")))
    })
}

pub fn render_user_website(website: Website) -> Result<Markup, AppError> {
    let website_name = website.name.ok_or_else(|| {
        tracing::error!("No name column or value is null");
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
    })?;

    let website_domain = website.domain.ok_or_else(|| {
        tracing::error!("No domain column or value is null");
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
    })?;

    Ok(maud::html! {
        section id="create-website" {
            h1 class="font-bold text-xl" {
                "Your website"
            }
            div {
            h2 {
                "Name: " (website_name)
            }
            h2 {
                "Domain: " (website_domain)
            }
            }
            button
            hx-get="/builder/contents/template"
            hx-trigger="click"
            hx-target="#contents"
            class="text-blue-500 underline"
            {
                "Next ->"
            }
        }
    })
}
