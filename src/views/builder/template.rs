use maud::Markup;
use reqwest::StatusCode;

use crate::{
    models::{
        error::AppError,
        template::{Template, TemplateType},
        website::Website,
    },
    views::builder::{
        head::render_main_builder_header,
        shared::{render_main_nav, render_sub_nav, MAIN_NAV, SUB_NAV},
    },
};

pub fn render_website_template_page() -> maud::Markup {
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
                            img class="w-auto h-12" src="/assets/images/builder/builder-logo.svg" alt="Your Company";
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
                            main id="contents" class="p-6" hx-get="/builder/contents/template" hx-trigger="load" {}
                        }
                    }
                }
            }
            div id="toast" {}
        }
    }
}

pub fn render_website_template(template_type: TemplateType) -> Markup {
    maud::html! {
        section id="choose-template" {
            h1 class="font-bold text-xl" {
                "Your template"
            }
            div class="flex gap-2" {
                "Type : "
                @match template_type {
                    TemplateType::RealEstate => {
                        div class="flex gap-2" {
                            "Real Estate"
                        }
                    },
                    TemplateType::Custom => "Custom",
                }
            }
            button
                hx-get="/builder/contents/data"
                hx-push-url="/builder/setup-data"
                hx-trigger="click"
                hx-target="#contents"
                class="text-blue-500 underline cursor-pointer"
            {
                "Next ->"
            }
        }
        (render_sub_nav(SUB_NAV, "Choose template", Some("outerHTML")))
    }
}

pub fn render_choose_template(
    website: Website,
    templates: Vec<Template>,
    authenticity_token: String,
) -> Result<Markup, AppError> {
    Ok(maud::html! {
        @let website_id = website.id.ok_or_else(|| {
            tracing::error!("No id column or value is null");
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        section id="choose-template" {
            @for template in templates {
                @let template_type = template.template_type.ok_or_else(|| {
                    tracing::error!("No template_type column or value is null");
                    AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
                })?;

                @let template_description = template.description.ok_or_else(|| {
                    tracing::error!("No description column or value is null");
                    AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
                })?;

                @let template_id = template.id.ok_or_else(|| {
                    tracing::error!("No id column or value is null");
                    AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
                })?;

                div class="relative flex flex-col border-slate-200 bg-white shadow-xs my-6 border rounded-lg w-96" {
                    div class="p-4" {
                        div class="flex justify-between items-center mb-2 w-full" {
                            h6 class="font-semibold text-slate-800 text-xl" {
                                @match template_type {
                                    TemplateType::RealEstate => "Real Estate",
                                    TemplateType::Custom => "Custom"
                                }
                            }
                            @if !matches!(template_type, TemplateType::Custom) {
                                a target="_blank" class="flex items-center gap-1 text-indigo-500 group" href="/" {
                                    div class="group" {
                                        "Preview"
                                    }
                                    img class="w-4 h-4" src="/assets/images/icon/arrow-next.svg" alt="next";
                                }
                            }
                        }
                        p class="text-slate-600" {
                            (template_description)
                        }
                    }
                    form
                        hx-post="/builder/website/template/select"
                        hx-swap="outerHMTL"
                        hx-target="#choose-template"
                    {
                        input type="hidden" name="authenticity_token" value=(authenticity_token);
                        input type="hidden" name="website_id" value=(website_id);
                        input type="hidden" name="template_id" value=(template_id);
                        div class="mt-2 px-4 pt-0 pb-4" {
                            button class="bg-indigo-500 hover:bg-indigo-400 focus:bg-indigo-400 active:bg-indigo-400 disabled:opacity-50 shadow-md hover:shadow-lg focus:shadow-none active:shadow-none disabled:shadow-none px-4 py-2 border border-transparent rounded-md w-full font-semibold text-center text-sm text-white transition-all cursor-pointer disabled:pointer-events-none" type="submit" {
                                "Select"
                            }
                        }
                    }
                }
            }
        }

        (render_sub_nav(SUB_NAV, "Choose template", Some("outerHTML")))
    })
}

pub fn render_no_website() -> Markup {
    maud::html! {
        div {
            "You don't have any website"
        }
        (render_sub_nav(SUB_NAV, "Choose template", Some("outerHTML")))
    }
}
