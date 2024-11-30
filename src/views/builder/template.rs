use maud::Markup;
use reqwest::StatusCode;

use crate::{
    models::{
        error::AppError,
        template::{Template, TemplateType},
        website::Website,
    },
    views::builder::home::{render_sub_nav, SUB_NAV},
};

pub fn render_website_template(template_type: TemplateType) -> Markup {
    maud::html! {
        section id="choose-template" {
            h1 class="font-bold text-xl" {
                "Your template"
            }
            div {
                h2 {
                    "Type : "
                    @match template_type {
                        TemplateType::RealEstate => "Real Estate",
                        TemplateType::Custom => "Custom",
                    }
                }
            }
            button
                hx-get="/builder/contents/data"
                hx-trigger="click"
                hx-target="#contents"
                class="text-blue-500 underline"
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

                div class="relative flex flex-col border-slate-200 bg-white shadow-sm my-6 border rounded-lg w-96" {
                    div class="p-4" {
                        div class="flex justify-between items-center mb-2 w-full" {
                            h6 class="font-semibold text-slate-800 text-xl" {
                                @match template_type {
                                    TemplateType::RealEstate => "Real Estate",
                                    TemplateType::Custom => "Custom"
                                }
                            }
                            @if !matches!(template_type, TemplateType::Custom) {
                                a class="flex items-center gap-2 text-indigo-500 group" href="/app/demo/realestate" {
                                    div class="group" {
                                        "Preview"
                                    }
                                    svg class="group-hover:h-4 group-hover:w-4 flex-0 w-0 h-0 transition-all translate-y-[1px]" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" {
                                        path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14 5l7 7m0 0l-7 7m7-7H3" {}
                                    }
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
                            button class="bg-indigo-500 hover:bg-indigo-400 focus:bg-indigo-400 active:bg-indigo-400 disabled:opacity-50 shadow-md hover:shadow-lg focus:shadow-none active:shadow-none disabled:shadow-none px-4 py-2 border border-transparent rounded-md w-full font-semibold text-center text-sm text-white transition-all disabled:pointer-events-none" type="submit" {
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
    }
}
