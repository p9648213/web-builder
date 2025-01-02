use maud::{html, Markup, DOCTYPE};

use crate::views::{
    builder::head::render_main_builder_header,
    icons::{footer_back_icon, footer_next_icon},
};

const MAIN_NAV_ITEM: [&str; 4] = ["CHOOSE STYLE", "BRANDING", "CONTENT", "ADVANCED SETTINGS"];
const CHOOSE_STYLE_NAV: [&str; 6] = [
    "Header",
    "Footer",
    "Home",
    "Search Result",
    "Property Details",
    "Contact",
];

pub fn render_edit_header_page() -> Markup {
    html! {
        (DOCTYPE)
        html class="h-full" {
          head {
            (render_main_builder_header())
          }
          body class="grid h-full" style="grid-template: auto 1fr auto / auto 1fr auto;" hx-boost="true" {
              title {
                  "Edit Header"
              }
              (render_nav_bar(0))
              main class="grid grid-cols-10 col-[1/4] grow" {
                div class="col-span-7 bg-[#F3F3F3]" {
                  (render_left_content())
                }
                div class="col-span-3" {
                  (render_right_content())
                }
              }
              (render_footer())
              div id="toast" {}
          }
        }
    }
}

pub fn render_nav_bar(highlight_index: usize) -> Markup {
    html! {
      nav class="z-10 shadow-md col-[1/4]" {
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

pub fn render_footer() -> Markup {
    html! {
      footer class="z-10 flex justify-between mt-auto px-6 py-4 col-[1/4]" style="box-shadow: rgba(67, 71, 85, 0.27) 0px 0px 0.25em, rgba(90, 125, 188, 0.05) 0px 0.25em 1em;" {
        div class="flex items-center gap-10" {
          button class="border-slate-300 px-12 py-2 border hover:border-blue-500 border-solid rounded-4xl text-lg hover:text-blue-500 transition-all duration-300 cursor-pointer" { "Preview" }
          button class="bg-blue-500 hover:bg-white px-12 py-2 border border-transparent hover:border-blue-500 rounded-4xl text-lg text-white hover:text-blue-500 transition-all duration-300 cursor-pointer" { "Publish" }
          div {
            button class="text-blue-500 text-lg cursor-pointer" { "View Site" }
          }
        }
        div class="flex items-center gap-10" {
          div {
            button class="flex items-center gap-2 text-black text-lg hover:text-gray-400 transition-all duration-300 cursor-pointer hover:stroke-gray-400 stroke-black"
            {
              (footer_back_icon()) "Back"
            }
          }
          div class="flex gap-2" {
            button class="flex items-center gap-2 bg-blue-500 hover:bg-white px-8 py-2 border border-transparent hover:border-blue-500 rounded-4xl text-lg text-white hover:text-blue-500 transition-all duration-300 cursor-pointer"
            {
              "Next" (footer_next_icon())}
            }
        }
      }
    }
}

pub fn render_left_content() -> Markup {
    html! {
      (render_sub_nav(0))
    }
}

pub fn render_right_content() -> Markup {
    html! {
      "Right Content"
    }
}

pub fn render_sub_nav(highlight_index: usize) -> Markup {
    html! {
      div class="flex justify-center p-8" {
        div class="flex gap-15 border-b border-b-gray-300 h-9 text-lg" {
          @for (index ,item) in CHOOSE_STYLE_NAV.into_iter().enumerate() {
            @if highlight_index == index {
              div class="border-b-2 border-b-blue-500 text-blue-500 cursor-pointer" { (item) }
            } @else {
              div class="border-b-2 border-b-transparent hover:border-b-blue-500 hover:text-blue-500 transition-all duration-300 cursor-pointer" { (item) }
            }
          }
        }
      }
    }
}
