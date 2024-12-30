use maud::{html, Markup, PreEscaped};

use crate::{
    models::rso_data::SearchProperty,
    views::{
        icons::{bath_icon_light, bed_icon_light, drop_down_icon, location_icon},
        real_estate::shared,
    },
};

//..............................................................................
//....SSSSSS....EEEEEEEEEE.....AAAA......RRRRRRRRR.......CCCCCC....HHH.....HHH..
//..SSSSSSSSS...EEEEEEEEEE.....AAAAA.....RRRRRRRRRRR...CCCCCCCCC...HHH.....HHH..
//..SSSSSSSSSS..EEEEEEEEEE.....AAAAA.....RRRRRRRRRRR..CCCCCCCCCCC..HHH.....HHH..
//..SSS...SSSS..EEE...........AAAAAA.....RRR.....RRR..CCCC...CCCC..HHH.....HHH..
//..SSSS........EEE...........AAAAAAA....RRR.....RRR..CCC.....CC...HHH.....HHH..
//..SSSSSSS.....EEEEEEEEEE...AAAA.AAA....RRRRRRRRRRR.CCCC..........HHHHHHHHHHH..
//...SSSSSSSS...EEEEEEEEEE...AAA..AAAA...RRRRRRRRRR..CCCC..........HHHHHHHHHHH..
//.....SSSSSSS..EEEEEEEEEE...AAAAAAAAA...RRRRRRRR....CCCC..........HHHHHHHHHHH..
//.........SSSS.EEE.........AAAAAAAAAA...RRR..RRRR....CCC.....CC...HHH.....HHH..
//.SSSS....SSSS.EEE.........AAAAAAAAAAA..RRR...RRRR...CCCC...CCCC..HHH.....HHH..
//..SSSSSSSSSS..EEEEEEEEEEE.AAA.....AAA..RRR....RRRR..CCCCCCCCCCC..HHH.....HHH..
//..SSSSSSSSSS..EEEEEEEEEEEAAAA.....AAAA.RRR....RRRR...CCCCCCCCC...HHH.....HHH..
//....SSSSSS....EEEEEEEEEEEAAA......AAAA.RRR.....RRRR....CCCCCC....HHH.....HHH..
//..............................................................................

pub fn render_search_box() -> Markup {
    html! {
      (PreEscaped(r#"
        <script type="module">
            import {scrollToTop} from "/assets/js/main.js";
            scrollToTop();
        </script>
      "#))
      div class="flex justify-center items-center mt-25" {
        div class="flex justify-center px-15 py-15 w-full max-w-360" {
          div class="gap-4 grid grid-cols-[4fr_4fr_3fr_3fr] grid-rows-[1fr_1fr] text-sm" {
            div class="flex items-center" {
              input class="border-slate-800 rounded-md placeholder:text-sm" type="search" placeholder="Search Ref ID";
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

//........................................................................
//.RRRRRRRRR....EEEEEEEEEE....SSSSSS....UUU.....UUU..LLL......LLTTTTTTTT..
//.RRRRRRRRRRR..EEEEEEEEEE..SSSSSSSSS...UUU.....UUU..LLL......LLTTTTTTTT..
//.RRRRRRRRRRR..EEEEEEEEEE..SSSSSSSSSS..UUU.....UUU..LLL......LLTTTTTTTT..
//.RRR.....RRR..EEE.........SSS...SSSS..UUU.....UUU..LLL..........TTTT....
//.RRR.....RRR..EEE.........SSSS........UUU.....UUU..LLL..........TTTT....
//.RRRRRRRRRRR..EEEEEEEEEE..SSSSSSS.....UUU.....UUU..LLL..........TTTT....
//.RRRRRRRRRR...EEEEEEEEEE...SSSSSSSS...UUU.....UUU..LLL..........TTTT....
//.RRRRRRRR.....EEEEEEEEEE.....SSSSSSS..UUU.....UUU..LLL..........TTTT....
//.RRR..RRRR....EEE................SSSS.UUU.....UUU..LLL..........TTTT....
//.RRR...RRRR...EEE........ESSS....SSSS.UUUU...UUUU..LLL..........TTTT....
//.RRR....RRRR..EEEEEEEEEEE.SSSSSSSSSS..UUUUUUUUUUU..LLLLLLLLLL...TTTT....
//.RRR....RRRR..EEEEEEEEEEE.SSSSSSSSSS...UUUUUUUUU...LLLLLLLLLL...TTTT....
//.RRR.....RRRR.EEEEEEEEEEE...SSSSSS......UUUUUUU....LLLLLLLLLL...TTTT....
//........................................................................

pub fn render_search_result(page: Option<u32>) -> Markup {
    let hx_get = if let Some(page) = page {
        format!("/rso/search-results?page={}", page)
    } else {
        "/rso/search-results".to_string()
    };

    html! {
      div
        hx-get=(hx_get)
        hx-target="#search-results"
        hx-trigger="load"
        class="flex justify-center items-center"
      {
        div id="search-results" class="flex flex-col justify-center items-center gap-10 px-15 pb-15 w-full max-w-360" {
          "Loading..."
        }
      }
    }
}

pub fn render_property_grids(
    properties: &Vec<SearchProperty>,
    property_count: u32,
    properties_per_page: u32,
    page_no: u32,
    listing_type: &str,
) -> Markup {
    let page_size = (property_count as f64 / properties_per_page as f64).ceil();

    html! {
      (PreEscaped(r#"
      <script type="module">
          import {scrollToTop} from "/assets/js/main.js";
          import {setupPropertyPictureSlider} from "/assets/js/app/slider.js";
          setupPropertyPictureSlider();
          scrollToTop();
      </script>
    "#))
      div class="flex justify-end w-full" {
        div class="text-xl" { (property_count) " properties" }
      }

      div class="gap-9 grid grid-cols-4" {
        @for property in properties {
          (shared::render_property_card(property, listing_type))
        }
      }
      div class="flex justify-center bg-white mt-6 p-2 rounded-full" {
        (render_pagination(page_size as u32, page_no))
      }
    }
}

pub fn render_pagination(total_pages: u32, page: u32) -> Markup {
    let mut before_page = page - 1;
    let mut after_page = page + 1;

    if page == total_pages {
        before_page = before_page - 2;
    } else if page == total_pages - 1 {
        before_page = before_page - 1;
    }

    if page == 1 {
        after_page = after_page + 2;
    } else if page == total_pages - 1 {
        before_page = before_page + 1;
    }

    if page == 1 {
        after_page = after_page + 2;
    } else if page == 2 {
        after_page = after_page + 1;
    }

    html! {
      ul class="flex" {
        @if page > 1 {
          li
            hx-get=(format!("/rso/search-results?page={}", page - 1))
            hx-push-url=(format!("/search-results?page={}", page - 1))
            hx-target="#search-results"
            hx-trigger="click"
            class="hover:bg-blue-500 px-5 rounded-md font-medium text-center text-lg hover:text-white leading-[45px] transition-all duration-300 cursor-pointer list-none ease-in-out"
          {
            span { (PreEscaped("&#x276E;")) }
          }
        }

        @if page > 2 {
          li
            hx-get="/rso/search-results?page=1"
            hx-push-url="/search-results?page=1"
            hx-target="#search-results"
            hx-trigger="click"
            class="hover:bg-blue-500 mx-1 rounded-md w-[45px] h-[45px] font-medium text-center text-lg hover:text-white leading-[45px] transition-all duration-300 cursor-pointer list-none ease-in-out"
          {
            span { "1" }
          }
          @if page > 3 {
            li class="text-center text-xl leading-[45px] cursor-default list-none" { span { "..." } }
          }
        }

        @for page_length in before_page..=after_page {
          @if page_length <= total_pages && page_length != 0 {
            @if page == page_length {
              li
                class="bg-blue-500 mx-1 rounded-md w-[45px] h-[45px] font-medium text-center text-lg text-white leading-[45px] cursor-pointer list-none"
              {
                span { (page_length) }
              }
            } @else {
              li
                hx-get=(format!("/rso/search-results?page={}", page_length))
                hx-push-url=(format!("/search-results?page={}", page_length))
                hx-target="#search-results"
                hx-trigger="click"
                class="hover:bg-blue-500 mx-1 rounded-md w-[45px] h-[45px] font-medium text-center text-lg hover:text-white leading-[45px] transition-all duration-300 cursor-pointer list-none ease-in-out"
              {
                span { (page_length) }
              }
            }
          }
        }

        @if page < total_pages - 1 {
          @if page < total_pages - 2 {
            li class="text-center text-xl leading-[45px] cursor-default list-none" { span { "..." } }
          }
          li
            hx-get=(format!("/rso/search-results?page={}", total_pages))
            hx-push-url=(format!("/search-results?page={}", total_pages))
            hx-target="#search-results"
            hx-trigger="click"
            class="hover:bg-blue-500 mx-1 rounded-md w-[45px] h-[45px] font-medium text-center text-lg hover:text-white leading-[45px] transition-all duration-300 cursor-pointer list-none ease-in-out"
          {
            span { (total_pages) }
          }
        }

        @if page < total_pages {
          li
            hx-get=(format!("/rso/search-results?page={}", page + 1))
            hx-push-url=(format!("/search-results?page={}", page + 1))
            hx-target="#search-results"
            hx-trigger="click"
            class="hover:bg-blue-500 px-5 rounded-md font-medium text-center text-lg hover:text-white leading-[45px] transition-all duration-300 cursor-pointer list-none ease-in-out"
          {
            span { (PreEscaped("&#x276F;")) }
          }
        }
      }
    }
}
