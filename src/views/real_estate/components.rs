use maud::{html, Markup, PreEscaped};

use crate::views::icons::{drop_down_icon, mail_icon, phone_icon};

pub fn render_nav_bar() -> Markup {
    html! {
      (PreEscaped(r#"
          <script type="module">
              import {showHideTopBarWhenScroll} from "/assets/js/builder/navbar.js";
              showHideTopBarWhenScroll();
          </script>
      "#))
      header id="navbar" class="top-0 right-0 left-0 z-50 fixed transition-all duration-500" {
        div class="flex justify-end gap-6 bg-white px-4 py-2 font-semibold text-sm" {
          div class="flex items-center gap-2" {
            (mail_icon())
            div class="flex gap-2" {
              span {"hanatest0102@gmail.com"}
              span {"|"}
              span {"nguyenhan0696@gmail.com"}
            }
          }
          div class="flex items-center gap-2" {
            (phone_icon())
            div class="flex gap-2" {
              span {"+34 0973477994"}
              span {"|"}
              span {"+34 39652874"}
            }
          }
        }
        div class="justify-center items-center gap-15 grid grid-cols-[auto_auto] bg-blue-500 px-4 py-4 h-full" {
          div class="h-10" {
            img class="h-full" src="https://cdn.resales-online.com/public/804pf2s7h1/agencies/3/3.jpg";
          }
          div class="flex items-center gap-10 h-full font-[500] text-white" {
            a class="hover:opacity-80" href="#" {"Home"}
            div class="relative flex items-center h-full group" {
              div class="flex items-end gap-1 hover:opacity-80 cursor-pointer" {
                span {"About us"}
                (drop_down_icon())
              }
              div class="group-hover:visible top-10 z-10 absolute flex flex-col gap-3 bg-blue-400 opacity-0 group-hover:opacity-100 px-3 py-2 rounded-md max-h-0 group-hover:max-h-30 whitespace-nowrap transition-all duration-500 invisible overflow-hidden" {
                a href="#" class="hover:opacity-80" {"Contact us"}
                a href="#" class="hover:opacity-80" {"About us"}
                a href="#" class="hover:opacity-80" {"Meet the team"}
              }
            }
            a class="hover:opacity-80" href="#" {"Sell your propery"}
            div class="relative flex items-center h-full group" {
              div class="flex items-end gap-1 hover:opacity-80 cursor-pointer" {
                span {"For sale"}
                (drop_down_icon())
              }
              div class="group-hover:visible top-10 z-10 absolute flex flex-col gap-3 bg-blue-400 opacity-0 group-hover:opacity-100 px-3 py-2 rounded-md max-h-0 group-hover:max-h-24 whitespace-nowrap transition-all duration-500 invisible overflow-hidden" {
                a href="#" class="hover:opacity-80" {"Resales"}
                a href="#" class="hover:opacity-80" {"New development"}
              }
            }
            div class="relative flex items-center h-full group" {
              div class="flex items-end gap-1 hover:opacity-80 cursor-pointer" {
                span {"For rent"}
                (drop_down_icon())
              }
              div class="group-hover:visible top-10 z-10 absolute flex flex-col gap-3 bg-blue-400 opacity-0 group-hover:opacity-100 px-3 py-2 rounded-md max-h-0 group-hover:max-h-24 whitespace-nowrap transition-all duration-500 invisible overflow-hidden" {
                a href="#" class="hover:opacity-80" {"Short rental"}
                a href="#" class="hover:opacity-80" {"Long rental"}
              }
            }
          }
        }
      }
    }
}

pub fn render_home_banner() -> Markup {
    html! {
      div class="flex justify-center items-center" {
        div class="relative bg-[url('https://d1qawt2l8egll1.cloudfront.net/prod/images/240825202440-copy-of-moving-to-spain-from-the-uk.png')] bg-cover w-full h-screen" {
          div class="top-1/4 left-1/2 absolute flex flex-col gap-4 bg-white opacity-80 p-6 rounded-xl max-w-120 transform -translate-x-1/2" {
            h2 class="font-semibold text-4xl" {
              "Find your new home"
            }

            p {
              "I'm a versatile paragraph. Add your own text and effortlessly customize me to make it your own. Feel free to edit and personalize your unique content!"
            }
          }
        }
      }
    }
}
