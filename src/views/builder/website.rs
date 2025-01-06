use maud::Markup;
use reqwest::StatusCode;

use crate::{
    models::{error::AppError, website::Website},
    views::builder::{
        head::render_main_builder_header,
        shared::{render_main_nav, render_sub_nav, MAIN_NAV, SUB_NAV},
    },
};

pub fn render_create_website_page() -> maud::Markup {
    maud::html! {
        (maud::DOCTYPE)
        head {
          (render_main_builder_header())
        }
        body hx-boost="true" {
            title {
                "General Setup"
            }
            div class="flex w-full h-full" {
                div class="xl:z-50 xl:fixed xl:inset-y-0 xl:flex xl:flex-col border-gray-200 border-r xl:w-72" {
                    div class="flex flex-col gap-y-5 px-6 ring-1 ring-white/5 overflow-y-auto grow" {
                        div class="flex items-center h-16 shrink-0" {
                            img class="w-auto h-12" src="/assets/images/builder-logo.svg" alt="Your Company";
                        }
                        (render_main_nav(MAIN_NAV, "Basic Setup", None))
                    }
                }
                div class="xl:pl-72 w-full" {
                    main {
                        h1 class="sr-only" {
                            "Account Settings"
                        }
                        header class="border-white/5 border-b" {
                            nav id="sub-nav" {}
                            main id="contents" class="p-6" hx-get="/builder/contents/website" hx-trigger="load" {}
                        }
                    }
                }
            }
            div id="toast" {}
        }
    }
}

pub fn render_create_website(
    authenticity_token: String,
    website: Option<Website>,
) -> Result<Markup, AppError> {
    Ok(maud::html! {
        section id="create-website" {
            @if let Some(website) = website {
                (render_user_website(&website)?)
            } @else {
                form
                    hx-post="/builder/website/create"
                    hx-target="#create-website"
                    hx-swap="outerHTML"
                    class="flex flex-col gap-2 max-w-72"
                {
                    input type="hidden" name="authenticity_token" value=(authenticity_token);
                    label { "Website name: " }
                    input type="text" name="name" {}
                    label { "Domain: " }
                    input type="text" name="domain" {}
                    button class="border-gray-500 mt-3 border cursor-pointer" {
                        "Create"
                    }
                }
            }
          }
        (render_sub_nav(SUB_NAV, "Create website", Some("outerHTML")))
    })
}

pub fn render_user_website(website: &Website) -> Result<Markup, AppError> {
    let website_id = website.id.ok_or_else(|| {
        tracing::error!("No id column or value is null");
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
    })?;

    let website_name = website.name.as_ref().ok_or_else(|| {
        tracing::error!("No name column or value is null");
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
    })?;

    let website_domain = website.domain.as_ref().ok_or_else(|| {
        tracing::error!("No domain column or value is null");
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
    })?;

    let builder_link = format!(
        "http://{}/builder/edit/{}/style/header",
        website_domain, website_id
    );

    let website_domain = format!("http://{}", website_domain);

    Ok(maud::html! {
        section id="create-website" {
            h1 class="font-bold text-xl" {
                "Your website"
            }
            div {
                h2 {
                    "Name: " (website_name)
                }
                h2 class="flex gap-2" {
                    "Domain: "
                    a target="_blank" class="flex items-center gap-1 text-indigo-500 group" href=(website_domain) {
                        div class="group" {
                            (website_domain)
                        }
                    }
                }
                h2 class="flex gap-2" {
                    "Builder Link: "
                    a target="_blank" class="flex items-center gap-1 text-indigo-500 group" href=(builder_link) {
                        div class="group" {
                            (builder_link)
                        }
                    }
                }
            }
            button
                hx-get="/builder/contents/template"
                hx-push-url="/builder/select-template"
                hx-trigger="click"
                hx-target="#contents"
                class="text-blue-500 underline cursor-pointer"
            {
                "Next ->"
            }
        }
    })
}
