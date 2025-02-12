use maud::{html, Markup, PreEscaped};
use reqwest::StatusCode;

use crate::{
    controllers::real_estate::pages::SearchQuery,
    models::{
        error::AppError,
        rso_data::{SearchProperty, TextOrNum},
    },
    views::real_estate::home,
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
pub fn render_search_box_1(search_query: &SearchQuery) -> Markup {
    let listing_type_url = if let Some(listing_type) = &search_query.listing_type {
        format!("/data/real-estate/listing-type?type={}", listing_type).to_owned()
    } else {
        "/data/real-estate/listing-type".to_owned()
    };

    html! {
      (PreEscaped(r#"
        <script type="module">
            import {scrollToTop} from "/assets/js/main.js";
            import {setupMarginNavbar} from "/assets/js/app/navbar.js";
            import {setupDropdown} from "/assets/js/app/searchbox.js";
            scrollToTop();
            setupMarginNavbar("search-results");
            setupDropdown();
        </script>
      "#))
      div class="flex flex-col justify-center items-center gap-6 shadow-lg p-6 py-8 rounded-3xl" {
        div class="justify-center grid grid-cols-[2fr_2fr_2fr_2fr_2fr_1fr_1fr] w-full" {
          div class="flex justify-center items-center" {
            input class="border-slate-800 rounded-md w-3/4 h-10 placeholder:text-sm" type="search" placeholder="Search Ref ID" ;
          }
          (home::render_search_box_selection_1("Listing Type", listing_type_url.as_str(), "listing-type-dropdown", "listing-type-label"))
          (home::render_search_box_selection_1("Location", "/rso/location", "location-dropdown", "location-label"))
          (home::render_search_box_selection_1("Property Types", "/rso/property-types", "property-types-dropdown", "property-types-label"))
          (home::render_search_box_selection_1("Price", "/data/real-estate/prices", "price-dropdown", "price-label"))
          (home::render_search_box_selection_1("Bath", "/data/real-estate/baths", "bath-dropdown", "bath-label"))
          (home::render_search_box_selection_1("Bed", "/data/real-estate/beds", "bed-dropdown", "bed-label"))
        }
      }
    }
}

pub fn render_search_box_2() -> Markup {
    html! {
      (PreEscaped(r#"
        <script type="module">
            import {scrollToTop} from "/assets/js/main.js";
            import {setupMarginNavbar} from "/assets/js/app/navbar.js";
            scrollToTop();
            setupMarginNavbar("search-results");
        </script>
      "#))
      div class="w-80 text-sm shrink-0" {
        div class="flex flex-col gap-3" {
          div class="flex justify-center items-center" {
            input class="py-2 border-slate-800 rounded-md w-full placeholder:text-sm" type="search" placeholder="Search Ref ID" ;
          }
          div class="flex justify-between items-center px-3 py-2.5 border border-black rounded-md" {
            span { "For Sales" }
            img class="w-4 h-4" src="/assets/images/icon/dropdown.svg" alt="dropdown";
          }
          div class="flex gap-3" {
            div class="flex flex-1 justify-between items-center px-3 py-2.5 border border-black rounded-md" {
              span { "Check in" }
              img class="w-4 h-4" src="/assets/images/icon/dropdown.svg" alt="dropdown";
            }
            div class="flex flex-1 justify-between items-center px-3 py-2.5 border border-black rounded-md" {
              span { "Check out" }
              img class="w-4 h-4" src="/assets/images/icon/dropdown.svg" alt="dropdown";
            }
          }
          div class="flex gap-3" {
            div class="flex flex-1 justify-between items-center px-3 py-2.5 border border-black rounded-md" {
              span { "Beds" }
              img class="w-4 h-4" src="/assets/images/icon/dropdown.svg" alt="dropdown";
            }
            div class="flex flex-1 justify-between items-center px-3 py-2.5 border border-black rounded-md" {
              span { "Baths" }
              img class="w-4 h-4" src="/assets/images/icon/dropdown.svg" alt="dropdown";
            }
          }
          div class="flex justify-between items-center px-3 py-2.5 border border-black rounded-md" {
            span { "All locations" }
            img class="w-4 h-4" src="/assets/images/icon/dropdown.svg" alt="dropdown";
          }
          div class="flex justify-between items-center px-3 py-2.5 border border-black rounded-md" {
            span { "All property types" }
            img class="w-4 h-4" src="/assets/images/icon/dropdown.svg" alt="dropdown";
          }
          div class="flex gap-3" {
            div class="flex flex-1 justify-between items-center px-3 py-2.5 border border-black rounded-md" {
              span { "Min" }
              span { "€" }
            }
            div class="flex flex-1 justify-between items-center px-3 py-2.5 border border-black rounded-md" {
              span { "Max" }
              span { "€" }
            }
          }
        }
      }
    }
}

pub fn render_search_box_3() -> Markup {
    html! {
      (PreEscaped(r#"
        <script type="module">
            import {scrollToTop} from "/assets/js/main.js";
            import {setupMarginNavbar} from "/assets/js/app/navbar.js";
            scrollToTop();
            setupMarginNavbar("search-results");
        </script>
      "#))
      div class="flex flex-col justify-center items-center gap-6 shadow-lg p-6 py-8 rounded-3xl w-full" {
        div class="flex justify-center gap-7 w-full max-w-360" {
          div class="flex justify-center items-center gap-2 px-5 border border-black rounded-3xl" {
            "Listing Type"
            div class="translate-y-0.5" {
              img class="w-4 h-4" src="/assets/images/icon/dropdown.svg" alt="dropdown";
            }
          }
          div class="flex justify-center items-center gap-2 px-5 border border-black rounded-3xl" {
            "Location"
            div class="translate-y-0.5" {
              img class="w-4 h-4" src="/assets/images/icon/dropdown.svg" alt="dropdown";
            }
          }
          div class="flex justify-center items-center gap-2 px-5 border border-black rounded-3xl" {
            "Property Type"
            div class="translate-y-0.5" {
              img class="w-4 h-4" src="/assets/images/icon/dropdown.svg" alt="dropdown";
            }
          }
          div class="flex justify-center items-center gap-2 px-5 border border-black rounded-3xl" {
            "Price"
            div class="translate-y-0.5" {
              img class="w-4 h-4" src="/assets/images/icon/dropdown.svg" alt="dropdown";
            }
          }
          div class="flex justify-center items-center gap-2 px-5 border border-black rounded-3xl" {
            "Bed"
            div class="translate-y-0.5" {
              img class="w-4 h-4" src="/assets/images/icon/dropdown.svg" alt="dropdown";
            }
          }
          div class="flex justify-center items-center gap-2 px-5 border border-black rounded-3xl" {
            "Bath"
            div class="translate-y-0.5" {
              img class="w-4 h-4" src="/assets/images/icon/dropdown.svg" alt="dropdown";
            }
          }
          div class="flex justify-center items-center" {
            input class="border-slate-800 rounded-md h-12 placeholder:text-sm" type="search" placeholder="Search Ref ID" ;
          }

          button
            class="bg-blue-500 hover:bg-blue-400 px-13 py-3 rounded-md font-semibold text-white cursor-pointer"
          {
            "Search"
          }

        }
      }
    }
}

pub fn render_search_box_4() -> Markup {
    let bed_icon_light = html! {
        img class="w-5 h-5" alt="bed" src="/assets/images/icon/bed-light.svg";
    };

    let bath_icon_light = html! {
        img class="w-5 h-5" alt="bath" src="/assets/images/icon/bath-light.svg";
    };

    let location_icon = html! {
        img class="w-5 h-5" alt="location" src="/assets/images/icon/location.svg";
    };

    html! {
      (PreEscaped(r#"
        <script type="module">
            import {scrollToTop} from "/assets/js/main.js";
            import {setupMarginNavbar} from "/assets/js/app/navbar.js";
            scrollToTop();
            setupMarginNavbar("search-results");
        </script>
      "#))
      div class="flex justify-center items-center w-full max-w-360" {
        div class="flex justify-center px-15 py-15 w-full" {
          div class="gap-4 grid grid-cols-[4fr_4fr_3fr_3fr] grid-rows-[1fr_1fr] w-full text-sm" {
            div class="flex items-center" {
              input class="border-slate-800 rounded-md w-full placeholder:text-sm" type="search" placeholder="Search Ref ID";
            }
            (render_selection_box_4("All Locations", Some(location_icon)))
            (render_selection_box_4("Any", Some(bed_icon_light)))
            (render_selection_box_4("Any", Some(bath_icon_light)))
            (render_selection_box_4("Resales", None))
            (render_selection_box_4("All Property Type", None))
            (render_selection_box_4("Min €", None))
            (render_selection_box_4("Max €", None))
          }
        }
      }
    }
}

pub fn render_selection_box_4(label: &str, icon: Option<Markup>) -> Markup {
    html! {
      div class="flex justify-between items-center px-3 py-2 border border-slate-800 border-solid rounded-md" {
        div class="flex items-center gap-2" {
          @if let Some(icon) = icon {
            (icon)
          }
          div {
            (label)
          }
        }
        img class="w-4 h-4" src="/assets/images/icon/dropdown.svg" alt="dropdown";
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

pub fn render_search_result_1(search_query: &SearchQuery) -> Result<Markup, AppError> {
    let mut params = vec![];

    if let Some(listing_type) = &search_query.listing_type {
        params.push(("listing_type", listing_type.to_owned()));
    } else {
        params.push(("listing_type", "resales".to_owned()));
    }

    if let Some(page) = &search_query.page {
        params.push(("page", page.to_string()));
    }

    let replace_url =
        reqwest::Url::parse_with_params("https://example.com/search-results", &params).map_err(
            |error| {
                tracing::error!("Failed to parse search result params: {}", error);
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
            },
        )?;

    params.push(("theme", "1".to_owned()));

    let hx_get_url =
        reqwest::Url::parse_with_params("https://example.com/rso/search-results", &params)
            .map_err(|error| {
                tracing::error!("Failed to parse search result params: {}", error);
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
            })?;

    Ok(html! {
      div
        hx-get=(hx_get_url.as_str().replace("https://example.com", ""))
        hx-target="#search-results"
        hx-trigger="load"
        hx-replace-url=(replace_url.as_str().replace("https://example.com", ""))
        class="flex justify-center items-center my-15"
      {
        div id="search-results" class="flex flex-col justify-center items-center gap-10 px-15 pb-15 w-full max-w-360" {
          "Loading..."
        }
      }
    })
}

pub fn render_search_result_2(page: Option<u32>) -> Markup {
    let hx_get = if let Some(page) = page {
        format!("/rso/search-results?page={}&theme=2", page)
    } else {
        "/rso/search-results?theme=2".to_string()
    };

    html! {
      div
        hx-get=(hx_get)
        hx-target="#search-results"
        hx-trigger="load"
        class="flex justify-center items-center w-full"
      {
        div id="search-results" class="flex flex-col justify-center items-center gap-10 w-full" {
          "Loading..."
        }
      }
    }
}

pub fn render_search_result_3(page: Option<u32>) -> Markup {
    let hx_get = if let Some(page) = page {
        format!("/rso/search-results?page={}&theme=3", page)
    } else {
        "/rso/search-results?theme=3".to_string()
    };

    html! {
      div
        hx-get=(hx_get)
        hx-target="#search-results"
        hx-trigger="load"
        class="flex justify-center items-center my-15"
      {
        div id="search-results" class="flex flex-col justify-center items-center gap-10 px-15 w-full max-w-360" {
          "Loading..."
        }
      }
    }
}

pub fn render_search_result_4(page: Option<u32>) -> Markup {
    let hx_get = if let Some(page) = page {
        format!("/rso/search-results?page={}&theme=4", page)
    } else {
        "/rso/search-results?theme=4".to_string()
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

//................................................
//.....GGGGGG.....RRRRRRRRR....III..DDDDDDDDD.....
//...GGGGGGGGGG...RRRRRRRRRRR..III..DDDDDDDDDD....
//...GGGGGGGGGGG..RRRRRRRRRRR..III..DDDDDDDDDDD...
//..GGGG....GGGG..RRR.....RRR..III..DDD....DDDD...
//..GGG......GG...RRR.....RRR..III..DDD.....DDD...
//.GGGG...........RRRRRRRRRRR..III..DDD.....DDDD..
//.GGGG...GGGGGG..RRRRRRRRRR...III..DDD.....DDDD..
//.GGGG...GGGGGG..RRRRRRRR.....III..DDD.....DDDD..
//..GGG...GGGGGG..RRR..RRRR....III..DDD.....DDD...
//..GGGG.....GGG..RRR...RRRR...III..DDD....DDDD...
//...GGGGGGGGGGG..RRR....RRRR..III..DDDDDDDDDDD...
//...GGGGGGGGGG...RRR....RRRR..III..DDDDDDDDDD....
//.....GGGGGG.....RRR.....RRRR.III..DDDDDDDDD.....
//................................................

pub fn render_property_grids_1(
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
          (render_property_card_1(property, listing_type))
        }
      }
      div class="flex justify-center bg-white mt-6 p-2 rounded-full" {
        (render_pagination(page_size as u32, page_no, 1, listing_type))
      }
    }
}

pub fn render_property_grids_2(
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

      div class="gap-9 grid grid-cols-2" {
        @for property in properties {
          (render_property_card_2(property, listing_type))
        }
      }
      div class="right-0 bottom-7 left-0 absolute flex justify-center bg-white m-auto p-2 rounded-full w-fit" {
        (render_pagination(page_size as u32, page_no, 2, listing_type))
      }
    }
}

pub fn render_property_grids_3(
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

      div class="gap-15 grid grid-cols-2" {
        @for property in properties {
          (render_property_card_3(property, listing_type))
        }
      }
      div class="flex justify-center bg-white mt-6 p-2 rounded-full" {
        (render_pagination(page_size as u32, page_no, 3, listing_type))
      }
    }
}

pub fn render_property_grids_4(
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
          (render_property_card_4(property, listing_type))
        }
      }
      div class="flex justify-center bg-white mt-6 p-2 rounded-full" {
        (render_pagination(page_size as u32, page_no, 4, listing_type))
      }
    }
}

//............................................
//.PPPPPPPPP.....AAAA.........GGGGGG.....GII..
//.PPPPPPPPPP....AAAA.......GGGGGGGGGG...GII..
//.PPPPPPPPPP...AAAAAA.....AGGGGGGGGGG...GII..
//.PPP....PPPP..AAAAAA.....AGGG...GGGGG..GII..
//.PPP....PPPP..AAAAAAA...AAGG.....GGG...GII..
//.PPPPPPPPPP..AAAAAAAA...AAGG...........GII..
//.PPPPPPPPPP..AAA..AAA...AAGG...GGGGGG..GII..
//.PPPPPPPPP..PAAAAAAAAA..AAGG...GGGGGG..GII..
//.PPP........PAAAAAAAAA..AAGG...GGGGGG..GII..
//.PPP........PAAAAAAAAAA..AGGG.....GGG..GII..
//.PPP.......PPAA....AAAA..AGGGGGGGGGGG..GII..
//.PPP.......PPA......AAA...GGGGGGGGGG...GII..
//.PPP.......PPA......AAAA....GGGGGG.....GII..
//............................................

pub fn render_pagination(total_pages: u32, page: u32, theme: u32, listing_type: &str) -> Markup {
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
            hx-get=(format!("/rso/search-results?page={}&theme={}&listing_type={}", page - 1, theme, listing_type))
            hx-push-url=(format!("/search-results?page={}&listing_type={}", page - 1, listing_type))
            hx-target="#search-results"
            hx-trigger="click"
            class="hover:bg-blue-500 px-5 rounded-md font-medium hover:text-white text-lg text-center leading-[45px] transition-all duration-300 ease-in-out cursor-pointer list-none"
          {
            span { (PreEscaped("&#x276E;")) }
          }
        }

        @if page > 2 {
          li
            hx-get=(format!("/rso/search-results?page=1&theme={}&listing_type={}", theme, listing_type))
            hx-push-url=(format!("/search-results?page=1&listing_type={}", listing_type))
            hx-target="#search-results"
            hx-trigger="click"
            class="hover:bg-blue-500 mx-1 rounded-md w-[45px] h-[45px] font-medium hover:text-white text-lg text-center leading-[45px] transition-all duration-300 ease-in-out cursor-pointer list-none"
          {
            span { "1" }
          }
          @if page > 3 {
            li class="text-xl text-center leading-[45px] cursor-default list-none" { span { "..." } }
          }
        }

        @for page_length in before_page..=after_page {
          @if page_length <= total_pages && page_length != 0 {
            @if page == page_length {
              li
                class="bg-blue-500 mx-1 rounded-md w-[45px] h-[45px] font-medium text-white text-lg text-center leading-[45px] cursor-pointer list-none"
              {
                span { (page_length) }
              }
            } @else {
              li
                hx-get=(format!("/rso/search-results?page={}&theme={}&listing_type={}", page_length, theme, listing_type))
                hx-push-url=(format!("/search-results?page={}&listing_type={}", page_length, listing_type))
                hx-target="#search-results"
                hx-trigger="click"
                class="hover:bg-blue-500 mx-1 rounded-md w-[45px] h-[45px] font-medium hover:text-white text-lg text-center leading-[45px] transition-all duration-300 ease-in-out cursor-pointer list-none"
              {
                span { (page_length) }
              }
            }
          }
        }

        @if page < total_pages - 1 {
          @if page < total_pages - 2 {
            li class="text-xl text-center leading-[45px] cursor-default list-none" { span { "..." } }
          }
          li
            hx-get=(format!("/rso/search-results?page={}&theme={}&listing_type={}", total_pages, theme, listing_type))
            hx-push-url=(format!("/search-results?page={}&listing_type={}", total_pages, listing_type))
            hx-target="#search-results"
            hx-trigger="click"
            class="hover:bg-blue-500 mx-1 rounded-md w-[45px] h-[45px] font-medium hover:text-white text-lg text-center leading-[45px] transition-all duration-300 ease-in-out cursor-pointer list-none"
          {
            span { (total_pages) }
          }
        }

        @if page < total_pages {
          li
            hx-get=(format!("/rso/search-results?page={}&theme={}&listing_type={}", page + 1, theme, listing_type))
            hx-push-url=(format!("/search-results?page={}&listing_type={}", page + 1, listing_type))
            hx-target="#search-results"
            hx-trigger="click"
            class="hover:bg-blue-500 px-5 rounded-md font-medium hover:text-white text-lg text-center leading-[45px] transition-all duration-300 ease-in-out cursor-pointer list-none"
          {
            span { (PreEscaped("&#x276F;")) }
          }
        }
      }
    }
}

//.......................................................
//.....CCCCCC.......AAAA......RRRRRRRRR....DDDDDDDDD.....
//...CCCCCCCCC......AAAAA.....RRRRRRRRRRR..DDDDDDDDDD....
//..CCCCCCCCCCC.....AAAAA.....RRRRRRRRRRR..DDDDDDDDDDD...
//..CCCC...CCCC....AAAAAA.....RRR.....RRR..DDD....DDDD...
//..CCC.....CC.....AAAAAAA....RRR.....RRR..DDD.....DDD...
//.CCCC...........AAAA.AAA....RRRRRRRRRRR..DDD.....DDDD..
//.CCCC...........AAA..AAAA...RRRRRRRRRR...DDD.....DDDD..
//.CCCC...........AAAAAAAAA...RRRRRRRR.....DDD.....DDDD..
//..CCC.....CC...AAAAAAAAAA...RRR..RRRR....DDD.....DDD...
//..CCCC...CCCC..AAAAAAAAAAA..RRR...RRRR...DDD....DDDD...
//..CCCCCCCCCCC..AAA.....AAA..RRR....RRRR..DDDDDDDDDDD...
//...CCCCCCCCC..AAAA.....AAAA.RRR....RRRR..DDDDDDDDDD....
//.....CCCCCC...AAA......AAAA.RRR.....RRRR.DDDDDDDDD.....
//.......................................................

pub fn render_property_card_1(property: &SearchProperty, listing_type: &str) -> Markup {
    let mut total_pictures = 0;

    let render_main_image = if let Some(main_image) = &property.main_image {
        total_pictures = 1;
        html! {
          img class="w-full h-full pointer-events-none shrink-0" src=(main_image);
        }
    } else {
        html! {}
    };

    let render_images = if let Some(images) = &property.pictures {
        total_pictures = total_pictures + images.count;
        html! {
          @for picture in &images.picture {
            img class="w-full h-full pointer-events-none shrink-0" src=(picture.picture_url);
          }
        }
    } else {
        html! {}
    };

    let price = if listing_type == "resales" || listing_type == "new-development" {
        property
            .price
            .as_ref()
            .unwrap_or(&"".to_string())
            .to_owned()
    } else {
        let rental_price_1 = property.rental_price_1.unwrap_or(0);
        let rental_price_2 = property.rental_price_2.unwrap_or(0);

        if rental_price_1 == rental_price_2 {
            rental_price_1.to_string()
        } else {
            format!("{} - {}", rental_price_1, rental_price_2)
        }
    };

    html! {
      div class="relative flex flex-col flex-1 gap-2 shadow-md rounded-lg overflow-hidden picture-container" {
        div class="relative picture-slider-container" {
          div class="flex h-42 transition-transform duration-500 picture-slider" {
            input type="hidden" value=(total_pictures);
            (render_main_image)
            (render_images)
          }
          div class="bottom-2 left-[50%] absolute flex gap-2 max-w-18 overflow-hidden -translate-x-[50%] pictures-dots" {
            @for i in 0..total_pictures as u8 {
              @if i == 0 {
                div class="bg-blue-500 p-1 rounded-full cursor-pointer" {}
              } @else {
                div class="bg-blue-200 p-1 rounded-full cursor-pointer" {}
              }
            }
          }
        }
        div
          hx-get=(format!("/section/real-estate/contents/property?id={}&type={}", property.reference, listing_type))
          hx-push-url=(format!("/property?id={}&type={}", property.reference, listing_type))
          hx-trigger="click"
          hx-target="main"
          class="flex flex-col justify-between gap-2 px-3 py-2 h-full cursor-pointer"
        {
          div {
            div class="font-bold text-blue-500 text-lg" {
              (price) " €"
            }
            div class="font-bold" {
              @if property.newdev_name == "" {
                (property.property_type.name_type)
              }@else {
                (property.newdev_name)
              }
            }
          }
          div class="flex flex-col gap-2" {
            div class="text-sm" {
              (property.location)
            }
            div class="flex gap-4 text-sm" {
              div class="flex items-center gap-2" {
                img class="w-5 h-5" alt="bed" src="/assets/images/icon/bed.svg";
                (property.bedrooms)
              }
              div class="flex items-center gap-2" {
                img class="w-5 h-5" alt="bath" src="/assets/images/icon/bath.svg";
                (property.bathrooms)
              }
              div class="flex items-center gap-2" {
                img class="w-5 h-5" alt="buit size" src="/assets/images/icon/built-size.svg";
                @match &property.built {
                    TextOrNum::Text(built) => (built),
                    TextOrNum::Num(built) => (built),
                }
                @if property.dimensions == "Metres" {
                  "m²"
                }
              }
            }
          }
        }
      }
    }
}

pub fn render_property_card_2(property: &SearchProperty, listing_type: &str) -> Markup {
    let mut total_pictures = 0;

    let render_main_image = if let Some(main_image) = &property.main_image {
        total_pictures = 1;
        html! {
          img class="w-full h-full pointer-events-none shrink-0" src=(main_image);
        }
    } else {
        html! {}
    };

    let render_images = if let Some(images) = &property.pictures {
        total_pictures = total_pictures + images.count;
        html! {
          @for picture in &images.picture {
            img class="w-full h-full pointer-events-none shrink-0" src=(picture.picture_url);
          }
        }
    } else {
        html! {}
    };

    let price = if listing_type == "resales" || listing_type == "new-development" {
        property
            .price
            .as_ref()
            .unwrap_or(&"".to_string())
            .to_owned()
    } else {
        let rental_price_1 = property.rental_price_1.unwrap_or(0);
        let rental_price_2 = property.rental_price_2.unwrap_or(0);

        if rental_price_1 == rental_price_2 {
            rental_price_1.to_string()
        } else {
            format!("{} - {}", rental_price_1, rental_price_2)
        }
    };

    html! {
      div class="relative flex rounded-lg overflow-hidden picture-container" style="box-shadow: rgba(14, 30, 37, 0.12) 0px 2px 4px 0px, rgba(14, 30, 37, 0.32) 0px 2px 16px 0px;" {
        div class="relative flex-1 overflow-hidden picture-slider-container" {
          div class="flex h-55 transition-transform duration-500 picture-slider" {
            input type="hidden" value=(total_pictures);
            (render_main_image)
            (render_images)
          }
          div class="bottom-2 left-[50%] absolute flex gap-2 max-w-18 overflow-hidden -translate-x-[50%] pictures-dots" {
            @for i in 0..total_pictures as u8 {
              @if i == 0 {
                div class="bg-blue-500 p-1 rounded-full cursor-pointer" {}
              } @else {
                div class="bg-blue-200 p-1 rounded-full cursor-pointer" {}
              }
            }
          }
        }
        div class="flex-1" {
          div
          hx-get=(format!("/section/real-estate/contents/property?id={}&type={}", property.reference, listing_type))
          hx-push-url=(format!("/property?id={}&type={}", property.reference, listing_type))
          hx-trigger="click"
          hx-target="main"
          class="flex flex-col justify-between gap-2 px-3 py-2 h-full cursor-pointer"
          {
            div class="flex flex-col gap-2" {
              div class="font-bold" {
                @if property.newdev_name == "" {
                  (property.property_type.name_type)
                }@else {
                  (property.newdev_name)
                }
              }
              div class="font-bold text-blue-500 text-lg" {
                (price) " €"
              }
              div class="text-sm" {
                (property.location)
              }
            }
            div class="flex flex-col gap-2" {
              div class="flex gap-4 divide-x divide-black text-sm" {
                div class="flex flex-1 items-center gap-2" {
                  img class="w-5 h-5" alt="bed" src="/assets/images/icon/bed.svg";
                  (property.bedrooms) " Beds"
                }
                div class="flex flex-1 items-center gap-2" {
                  img class="w-5 h-5" alt="bath" src="/assets/images/icon/bath.svg";
                  (property.bathrooms) " Baths"
                }
              }
            }
          }
        }
      }
    }
}

pub fn render_property_card_3(property: &SearchProperty, listing_type: &str) -> Markup {
    let mut total_pictures = 0;

    let render_main_image = if let Some(main_image) = &property.main_image {
        total_pictures = 1;
        html! {
          img class="w-full h-full pointer-events-none shrink-0" src=(main_image);
        }
    } else {
        html! {}
    };

    let render_images = if let Some(images) = &property.pictures {
        total_pictures = total_pictures + images.count;
        html! {
          @for picture in &images.picture {
            img class="w-full h-full pointer-events-none shrink-0" src=(picture.picture_url);
          }
        }
    } else {
        html! {}
    };

    let price = if listing_type == "resales" || listing_type == "new-development" {
        property
            .price
            .as_ref()
            .unwrap_or(&"".to_string())
            .to_owned()
    } else {
        let rental_price_1 = property.rental_price_1.unwrap_or(0);
        let rental_price_2 = property.rental_price_2.unwrap_or(0);

        if rental_price_1 == rental_price_2 {
            rental_price_1.to_string()
        } else {
            format!("{} - {}", rental_price_1, rental_price_2)
        }
    };

    html! {
      div class="relative flex rounded-lg overflow-hidden picture-container" style="box-shadow: rgba(14, 30, 37, 0.12) 0px 2px 4px 0px, rgba(14, 30, 37, 0.32) 0px 2px 16px 0px;" {
        div class="relative flex-1 overflow-hidden picture-slider-container" {
          div class="flex h-60 transition-transform duration-500 picture-slider" {
            input type="hidden" value=(total_pictures);
            (render_main_image)
            (render_images)
          }
          div class="bottom-2 left-[50%] absolute flex gap-2 max-w-18 overflow-hidden -translate-x-[50%] pictures-dots" {
            @for i in 0..total_pictures as u8 {
              @if i == 0 {
                div class="bg-blue-500 p-1 rounded-full cursor-pointer" {}
              } @else {
                div class="bg-blue-200 p-1 rounded-full cursor-pointer" {}
              }
            }
          }
        }
        div class="flex-1" {
          div
          hx-get=(format!("/section/real-estate/contents/property?id={}&type={}", property.reference, listing_type))
          hx-push-url=(format!("/property?id={}&type={}", property.reference, listing_type))
          hx-trigger="click"
          hx-target="main"
          class="flex flex-col justify-between gap-2 px-3 py-2 h-full cursor-pointer"
          {
            div class="flex flex-col gap-2" {
              div class="font-bold" {
                @if property.newdev_name == "" {
                  (property.property_type.name_type)
                }@else {
                  (property.newdev_name)
                }
              }
              div class="font-bold text-blue-500 text-lg" {
                (price) " €"
              }
              div class="text-sm" {
                (property.location)
              }
            }
            div class="flex flex-col gap-2" {
              div class="flex gap-4 divide-x divide-black text-sm" {
                div class="flex flex-1 items-center gap-2" {
                  img class="w-5 h-5" alt="bed" src="/assets/images/icon/bed.svg";
                  (property.bedrooms) " Beds"
                }
                div class="flex flex-1 items-center gap-2" {
                  img class="w-5 h-5" alt="bed" src="/assets/images/icon/bed.svg";
                  (property.bathrooms) " Baths"
                }
              }
            }
          }
        }
      }
    }
}

pub fn render_property_card_4(property: &SearchProperty, listing_type: &str) -> Markup {
    let mut total_pictures = 0;

    let render_main_image = if let Some(main_image) = &property.main_image {
        total_pictures = 1;
        html! {
          img class="h-full pointer-events-none shrink-0" src=(main_image);
        }
    } else {
        html! {}
    };

    let render_images = if let Some(images) = &property.pictures {
        total_pictures = total_pictures + images.count;
        html! {
          @for picture in &images.picture {
            img class="w-full h-full pointer-events-none shrink-0" src=(picture.picture_url);
          }
        }
    } else {
        html! {}
    };

    let price = if listing_type == "resales" || listing_type == "new-development" {
        property
            .price
            .as_ref()
            .unwrap_or(&"".to_string())
            .to_owned()
    } else {
        let rental_price_1 = property.rental_price_1.unwrap_or(0);
        let rental_price_2 = property.rental_price_2.unwrap_or(0);

        if rental_price_1 == rental_price_2 {
            rental_price_1.to_string()
        } else {
            format!("{} - {}", rental_price_1, rental_price_2)
        }
    };

    html! {
      div class="relative flex flex-col gap-2 shadow-md rounded-lg overflow-hidden picture-container" {
        div class="relative picture-slider-container" {
          div class="flex h-42 transition-transform duration-500 picture-slider" {
            input type="hidden" value=(total_pictures);
            (render_main_image)
            (render_images)
          }
          div class="bottom-2 left-[50%] absolute flex gap-2 max-w-18 overflow-hidden -translate-x-[50%] pictures-dots" {
            @for i in 0..total_pictures as u8 {
              @if i == 0 {
                div class="bg-blue-500 p-1 rounded-full cursor-pointer" {}
              } @else {
                div class="bg-blue-200 p-1 rounded-full cursor-pointer" {}
              }
            }
          }
        }
        div
          hx-get=(format!("/section/real-estate/contents/property?id={}&type={}", property.reference, listing_type))
          hx-push-url=(format!("/property?id={}&type={}", property.reference, listing_type))
          hx-trigger="click"
          hx-target="main"
          class="flex flex-col justify-between gap-2 px-3 py-2 h-full cursor-pointer"
        {
          div {
            div class="font-bold text-blue-500 text-lg" {
              (price) " €"
            }
            div class="font-bold" {
              @if property.newdev_name == "" {
                (property.property_type.name_type)
              }@else {
                (property.newdev_name)
              }
            }
          }
          div class="flex flex-col gap-2" {
            div class="text-sm" {
              (property.location)
            }
            div class="flex gap-4 divide-x divide-black text-sm" {
              div class="flex flex-1 items-center gap-2" {
                img class="w-5 h-5" alt="bed" src="/assets/images/icon/bed.svg";
                (property.bedrooms) " Beds"
              }
              div class="flex flex-1 items-center gap-2" {
                img class="w-5 h-5" alt="bath" src="/assets/images/icon/bath.svg";
                (property.bathrooms) " Baths"
              }
            }
          }
        }
      }
    }
}
