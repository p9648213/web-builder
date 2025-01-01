use maud::{html, Markup, DOCTYPE};

use crate::views::builder::head::render_main_builder_header;

const MAIN_NAV_ITEM: [&str; 4] = ["CHOOSE STYLE", "BRANDING", "CONTENT", "ADVANCED SETTINGS"];

pub fn render_edit_header_page() -> Markup {
    html! {
        (DOCTYPE)
        head {
          (render_main_builder_header())
        }
        body hx-boost="true" {
            title {
                "Edit Header"
            }
            main {
              (render_nav_bar(0))
            }
            div id="toast" {}
        }
    }
}

pub fn render_nav_bar(highlight_index: usize) -> Markup {
    html! {
      nav class="shadow-md" {
        div class="flex p-6" {
          div class="w-fit h-8" {
            img class="h-full" src="/assets/images/builder-logo.svg" alt="logo" ;
          }
          div class="flex justify-center items-center w-full" {
            ul class="flex gap-36 font-bold" {
              @for (index, item) in MAIN_NAV_ITEM.into_iter().enumerate() {
                @if index == highlight_index {
                  li class="text-blue-500 cursor-pointer" { (item) }
                }@else {
                  li class="hover:text-blue-500 cursor-pointer" { (item) }
                }
              }
            }
          }
        }
      }
    }
}
