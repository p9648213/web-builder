use maud::{html, Markup, PreEscaped};

use crate::views::icons::{drop_down_icon, mail_icon, phone_icon};

pub fn render_nav_bar() -> Markup {
    html! {
      (PreEscaped(r#"
          <script type="module">
              import {showHideTopBarWhenScroll} from "/assets/js/app/navbar.js";
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
      div class="flex justify-center items-center w-full" {
        div class="relative bg-[url('https://d1qawt2l8egll1.cloudfront.net/prod/images/240825202440-copy-of-moving-to-spain-from-the-uk.png')] bg-cover w-full h-screen" {
          div class="top-1/4 left-1/2 absolute flex flex-col gap-4 bg-white p-6 rounded-xl max-w-120 transform -translate-x-1/2" {
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

pub fn render_selection_drop_down(choices: Vec<&str>, highlight: &str) -> Markup {
    html! {
      @for choice in choices {
        @if choice == highlight {
          div class="bg-blue-400 px-2 rounded-sm text-white" {
            (choice)
          }
        } @else {
          div class="hover:bg-blue-300 px-2 rounded-sm hover:text-white cursor-pointer" {
            (choice)
          }
        }
      }
    }
}

pub fn render_selection_label(label: &str, id: &str) -> Markup {
    html! {
      div id=(id) hx-swap-oob="outerHTML" class="text-slate-500 text-sm" {
        (label)
      }
    }
}

pub fn render_search_box_selection(
    title: &str,
    hx_get: &str,
    dropdown_id: &str,
    label_id: &str,
) -> Markup {
    let container_class = if title != "Bed" {
        "relative flex justify-center items-center border-r border-black border-solid"
    } else {
        "relative flex justify-center items-center"
    };

    html! {
      div
        hx-get=(hx_get)
        hx-swap="innerHTML"
        hx-trigger="load"
        hx-target=(format!("#{}-items", dropdown_id))
        class=(container_class)
      {
        div class="flex flex-col gap-4" {
          div class="flex" {
            div id=(dropdown_id) class="flex items-end gap-1 cursor-pointer" {
              span class="font-semibold" {(title)}
              (drop_down_icon())
            }
            div id=(format!("{}-items", dropdown_id)) class="top-7 right-1 absolute flex flex-col gap-1 bg-white opacity-0 shadow p-2 rounded-md h-0 transition-all duration-500 invisible overflow-hidden pointer-events-none dropdown" {
              "Loading..."
            }
          }
          div id=(label_id) hx-swap-oob="outerHTML" class="text-slate-500 text-sm" {
            "Loading..."
          }
        }
      }
    }
}

pub fn render_home_search_box() -> Markup {
    html! {
      (PreEscaped(r#"
        <script type="module">
            import {setupDropdown} from "/assets/js/app/searchbox.js";
            setupDropdown();
        </script>
      "#))
      div class="right-0 bottom-0 left-0 absolute flex justify-center items-center bg-[rgba(255,255,255,0.8)] p-6 h-50" {
        div class="grid grid-cols-[300px_170px_170px_170px_170px_100px_100px] w-full max-w-6xl" {
          div class="flex justify-end items-center pr-3" {
            input class="rounded-md w-3/4 h-10 placeholder:text-sm" type="search" placeholder="Search Ref ID" ;
          }
          (render_search_box_selection("Listing Type", "/rso/listing-type?demo=true", "listing-type-dropdown", "listing-type-label"))
          // (render_search_box_selection("Location"))
          // (render_search_box_selection("Property Types"))
          // (render_search_box_selection("Price"))
          // (render_search_box_selection("Bath"))
          // (render_search_box_selection("Bed"))
        }
      }
    }
}
