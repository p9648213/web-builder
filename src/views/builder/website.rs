use crate::{
    models::website::WebsiteDTO,
    views::builder::home::{render_sub_nav, SUB_NAV},
};

pub fn render_create_website(
    authenticity_token: String,
    website: Option<WebsiteDTO>,
) -> maud::Markup {
    maud::html! {
        section id="create-website" {
            @if let Some(website) = website {
                (render_user_website(website))
            } @else {
                form
                    hx-post="/builder/website/create"
                    hx-target="#create-website"
                    hx-swap="outerHTML"
                    class="flex flex-col"
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
    }
}

pub fn render_user_website(website: WebsiteDTO) -> maud::Markup {
    maud::html! {
        section id="create-website" {
            h1 class="font-bold text-xl" {
                "Your website"
            }
            div {
            h2 {
                "Name: " (website.name)
            }
            h2 {
                "Domain: " (website.domain)
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
    }
}
