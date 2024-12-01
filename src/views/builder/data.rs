use maud::PreEscaped;

use crate::{
    models::rso_data::RsoData,
    views::builder::home::{render_sub_nav, SUB_NAV},
};

pub fn render_setup_data(authenticity_token: String, rso_data: Option<RsoData>) -> maud::Markup {
    let checked = if rso_data.is_some() { Some(true) } else { None };

    let checkbox_value = if checked.is_some() { "true" } else { "false" };

    let rso_data =
        rso_data.unwrap_or_else(|| RsoData::new(None, None, None, None, None, None, None, None));

    maud::html! {
      (PreEscaped(r#"
        <script type="module">
            import {checkDataFormDropdown} from "/assets/js/builder/data.js";
            checkDataFormDropdown();
        </script>
      "#))

      section class="setup-data" {
        form
          class="flex items-center gap-2"
          hx-swap="none"
          hx-patch="/builder/website/data/rso-data/status"
          hx-trigger="change"
        {
          input type="hidden" name="authenticity_token" value=(authenticity_token);
          input
            type="checkbox"
            name="rso_data_status"
            id="rso-data-status"
            checked=[checked]
            value=(checkbox_value);
          label for="rso-data-status" {
            "Use Rso data"
          }
        }

        form
          id="create-data-form"
          hx-post="/builder/website/data/rso-data/details"
          hx-swap="none"
          class="flex-col gap-2 hidden max-w-72"
        {
          input type="hidden" name="authenticity_token" value=(authenticity_token);
          label {
            "Identifier Id: "
          }
          input required type="number" name="identifier_id" value=[rso_data.identifier_id];

          label {
            "Api Key: "
          }
          input required type="text" name="api_key" value=[rso_data.api_key];

          label {
            "Filter Id Sale: "
          }
          input required type="number" name="filter_id_sale" value=[rso_data.filter_id_sale];

          label {
            "Filter Id Long: "
          }
          input required type="number" name="filter_id_long" value=[rso_data.filter_id_long];

          label {
            "Filter Id Short: "
          }
          input required type="number" name="filter_id_short" value=[rso_data.filter_id_short];

          label {
            "Filter Id Featured: "
          }
          input required type="number" name="filter_id_featured" value=[rso_data.filter_id_featured];

          button class="border-gray-500 mt-3 border" {
            "Save"
          }
        }
      }
      (render_sub_nav(SUB_NAV, "Setup data", Some("outerHTML")))
    }
}
