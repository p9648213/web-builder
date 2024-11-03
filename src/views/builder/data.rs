use crate::views::builder::home::{render_sub_nav, SUB_NAV};

pub fn render_setup_data(authenticity_token: String) -> maud::Markup {
    maud::html! {
      section {
        form hx-post="/builder/data/data-source" hx-target="#data-view" class="flex flex-col" {
          input type="hidden" name="authenticity_token" value=(authenticity_token);
          label {
            "Data source: "
          }
          input type="text" name="url" {}
          button class="border-gray-500 mt-3 border" {
            "Get data"
          }
        }
        pre id="data-view" class="mt-12" {}
      }
      (render_sub_nav(SUB_NAV, "Setup Data", Some("outerHTML")))
    }
}
