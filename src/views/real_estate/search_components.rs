use maud::{html, Markup};

use crate::views::icons::{bath_icon_light, bed_icon_light, drop_down_icon, location_icon};

pub fn render_search_box() -> Markup {
    html! {
      div class="flex justify-center items-center mt-25" {
        div class="flex justify-center px-15 py-15 w-full max-w-360" {
          div class="gap-4 grid grid-cols-[4fr_4fr_3fr_3fr] grid-rows-[1fr_1fr] text-sm" {
            div class="flex items-center" {
              input class="rounded-md placeholder:text-sm" type="search" placeholder="Search Ref ID";
            }
            (render_selection_box("All Locations", Some(location_icon())))
            (render_selection_box("Any", Some(bed_icon_light())))
            (render_selection_box("Any", Some(bath_icon_light())))
            (render_selection_box("Resales", None))
            (render_selection_box("All Property Type", None))
            (render_selection_box("Min €", None))
            (render_selection_box("Max €", None))
          }
        }
      }
    }
}

pub fn render_selection_box(label: &str, icon: Option<Markup>) -> Markup {
    html! {
      div class="flex justify-between items-center border-slate-800 px-3 py-2 border border-solid rounded-md" {
        div class="flex items-center gap-2" {
          @if let Some(icon) = icon {
            (icon)
          }
          div {
            (label)
          }
        }
        (drop_down_icon())
      }
    }
}
