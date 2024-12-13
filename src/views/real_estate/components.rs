use maud::{html, Markup, PreEscaped};
use tailwind_fuse::tw_merge;

use crate::{
    models::rso_data::{LocationDynamic, PropertyType, ProvinceAreaDynamic},
    views::icons::{drop_down_icon, mail_icon, phone_icon},
};

//......................................
//.NNNN....NNN.....AAAA...AAAV.....VVV..
//.NNNN....NNN.....AAAAA...AAV.....VVV..
//.NNNNN...NNN.....AAAAA...AAVV....VVV..
//.NNNNN...NNN....AAAAAA...AAVV...VVVV..
//.NNNNNN..NNN....AAAAAAA...AVV...VVVV..
//.NNNNNNN.NNN...AAAA.AAA...AVVV..VVV...
//.NNN.NNN.NNN...AAA..AAAA..AVVV.VVVV...
//.NNN.NNNNNNN...AAAAAAAAA...VVV.VVVV...
//.NNN..NNNNNN..AAAAAAAAAA...VVVVVVV....
//.NNN..NNNNNN..AAAAAAAAAAA..VVVVVVV....
//.NNN...NNNNN..AAA.....AAA...VVVVV.....
//.NNN....NNNN.NAAA.....AAAA..VVVVV.....
//.NNN....NNNN.NAA......AAAA..VVVVV.....
//......................................

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

//..............................................................................
//.BBBBBBBBB.......AAAA......NNNN....NNN..NNNN....NNN..EEEEEEEEEE..RRRRRRRRR....
//.BBBBBBBBBBB.....AAAAA.....NNNN....NNN..NNNN....NNN..EEEEEEEEEE..RRRRRRRRRRR..
//.BBBBBBBBBBB.....AAAAA.....NNNNN...NNN..NNNNN...NNN..EEEEEEEEEE..RRRRRRRRRRR..
//.BBB.....BBB....AAAAAA.....NNNNN...NNN..NNNNN...NNN..EEE.........RRR.....RRR..
//.BBB.....BBB....AAAAAAA....NNNNNN..NNN..NNNNNN..NNN..EEE.........RRR.....RRR..
//.BBBBBBBBBB....AAAA.AAA....NNNNNNN.NNN..NNNNNNN.NNN..EEEEEEEEEE..RRRRRRRRRRR..
//.BBBBBBBBBB....AAA..AAAA...NNN.NNN.NNN..NNN.NNN.NNN..EEEEEEEEEE..RRRRRRRRRR...
//.BBBBBBBBBBB...AAAAAAAAA...NNN.NNNNNNN..NNN.NNNNNNN..EEEEEEEEEE..RRRRRRRR.....
//.BBB.....BBBB.AAAAAAAAAA...NNN..NNNNNN..NNN..NNNNNN..EEE.........RRR..RRRR....
//.BBB.....BBBB.AAAAAAAAAAA..NNN..NNNNNN..NNN..NNNNNN..EEE.........RRR...RRRR...
//.BBBBBBBBBBB..AAA.....AAA..NNN...NNNNN..NNN...NNNNN..EEEEEEEEEEE.RRR....RRRR..
//.BBBBBBBBBBB.BAAA.....AAAA.NNN....NNNN..NNN....NNNN..EEEEEEEEEEE.RRR....RRRR..
//.BBBBBBBBBB..BAA......AAAA.NNN....NNNN..NNN....NNNN..EEEEEEEEEEE.RRR.....RRR..
//..............................................................................

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
          div class="bg-blue-400 px-2 py-0.5 rounded-sm text-sm text-white" {
            (choice)
          }
        } @else {
          div class="hover:bg-blue-300 px-2 py-0.5 rounded-sm text-sm hover:text-white cursor-pointer" {
            (choice)
          }
        }
      }
    }
}

pub fn render_beds_baths_selection_drop_down(choices: Vec<&str>, highlight: &str) -> Markup {
    html! {
      div class="flex justify-center h-full text-sm gap-4 flex-col px-2" {
        div class="flex border border-slate-900 rounded-md h-10" {
          @for choice in choices {
            @if choice == highlight {
              div class="flex w-10.5 justify-center items-center border-r border-slate-900 first:rounded-tl-md first:rounded-bl-md last:border-none last:rounded-br-md last:rounded-tr-md bg-blue-400 text-white cursor-pointer" {
                (choice)
              }
            }@else {
              div class="flex w-10.5 justify-center items-center border-r border-slate-900 first:rounded-tl-md first:rounded-bl-md last:border-none last:rounded-br-md last:rounded-tr-md cursor-pointer hover:bg-blue-300 hover:text-white" {
                (choice)
              }
            }
          }
        }
        div class="flex gap-2 items-center" {
          input id="exact" class="rounded-sm" name="exact-checkbox" type="checkbox";
          label for="exact" {"Exact number"}
        }
      }
    }
}

pub fn render_location_selection_drop_down(
    provinces: ProvinceAreaDynamic,
    highlight: &str,
) -> Markup {
    html! {
      div class="flex flex-col gap-3" {
        @match provinces {
          ProvinceAreaDynamic::Single(province_area) => {
            @if highlight == "All" {
              @let id = format!("{}-location", province_area.province_area_name);
              (render_input_radio("All", "province", "all-location", Some(true)))
              (render_input_radio(province_area.province_area_name.as_str(), "province", id.as_str(), None))

              @match province_area.locations.location {
                LocationDynamic::Single(location) => {
                  @let id = format!("{}-child", location);
                  (render_check_box(location.as_str(), None, id.as_str(), None, None));
                },
                LocationDynamic::Multiple(locations) => {
                  @for location in locations {
                    @let id = format!("{}-child", location);
                    (render_check_box(location.as_str(), None, id.as_str(), None, None));
                  }
                },
              }

            } @else {
              @let id = format!("{}-location", province_area.province_area_name);

              (render_input_radio("All", "province", "all-location", None))

              @if province_area.province_area_name == highlight {
                (render_input_radio(province_area.province_area_name.as_str(), "province", id.as_str(), Some(true)))
              }@else {
                (render_input_radio(province_area.province_area_name.as_str(), "province", id.as_str(), None))
              }
            }
          },
          ProvinceAreaDynamic::Multiple(province_areas) => {
            @if highlight == "All" {
              (render_input_radio("All", "province", "all-location", Some(true)))
            } @else {
              (render_input_radio("All", "province", "all-location", None))
            }

            @for province_area in province_areas {
              @let id = format!("{}-location", province_area.province_area_name);
              (render_input_radio(province_area.province_area_name.as_str(), "province", id.as_str(), None))

              @match province_area.locations.location {
                LocationDynamic::Single(location) => {
                  @let id = format!("{}-child", location);
                  (render_check_box(location.as_str(), None, id.as_str(), None, None));
                },
                LocationDynamic::Multiple(locations) => {
                  @for location in locations {
                    @let id = format!("{}-child", location);
                    (render_check_box(location.as_str(), None, id.as_str(), None, None));
                  }
                },
              }
            }
          },
        }
      }
    }
}

pub fn render_property_types_selection_drop_down(property_types: Vec<PropertyType>) -> Markup {
    html! {
      div class="flex flex-col gap-3" {
        @for property_type in property_types {
          (render_check_box(property_type.prop_type.as_str(), Some(property_type.option_value.as_str()) , property_type.option_value.as_str(), None, Some("ml-0")));
          @for sub_type in property_type.sub_types {
            (render_check_box(sub_type.prop_sub_type.as_str(), Some(property_type.option_value.as_str()), sub_type.sub_type_option_value.as_str(), None, None));
          }
        }
      }
    }
}

pub fn render_input_radio(value: &str, name: &str, id: &str, checked: Option<bool>) -> Markup {
    html! {
      div class="flex items-center gap-2 text-sm" {
        input type="radio" checked=[checked] id=(id) name=(name) value=(value);
        label for=(id) {(value)}
      }
    }
}

pub fn render_check_box(
    name: &str,
    value: Option<&str>,
    id: &str,
    checked: Option<bool>,
    tw_class: Option<&str>,
) -> Markup {
    let class = if let Some(tw_class) = tw_class {
        tw_merge!("flex items-center gap-2 ml-5 text-sm", tw_class)
    } else {
        "flex items-center gap-2 ml-5 text-sm".to_string()
    };

    html! {
      div class=(class) {
        input class="rounded-sm" type="checkbox" name=(name) value=[value] id=(id) checked=[checked];
        label for=(id) {(name)}
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

pub fn render_price_input(id: &str) -> Markup {
    html! {
      (PreEscaped(r#"
        <script type="module">
            import {setupPriceInput} from "/assets/js/app/searchbox.js";
            setupPriceInput();
        </script>
      "#))
      div id=(id) hx-swap-oob="outerHTML" {
        label class="relative" {
          span class="absolute right-0 top-0 pr-2 text-slate-500 text-sm" {"€"}
          input id=(format!("{}-min", id)) placeholder="From" class="h-5 w-[50%] outline-none bg-transparent focus:ring-0 border-l-0 border-t-0 border-b-0 border-r border-r-slate-700 pr-4.5 placeholder:text-slate-500 placeholder:text-sm text-sm";
        }
        label class="relative" {
          span class="absolute right-0 top-0 pr-2 text-slate-500 text-sm" {"€"}
          input id=(format!("{}-max", id)) placeholder="To" class="h-5 w-[50%] border-none outline-none bg-transparent focus:ring-0 placeholder:text-slate-500 placeholder:text-sm pr-4.5 text-sm";
        }

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

    let dropdown_items_class = if title != "Listing Type" {
        "top-7 absolute flex flex-col gap-1 bg-white opacity-0 shadow p-2 rounded-md h-0 max-h-0 whitespace-pre transition-all duration-500 invisible pointer-events-none dropdown overflow-auto z-1"
    } else {
        "top-7 absolute flex flex-col gap-1 bg-white opacity-0 shadow p-2 rounded-md h-0 whitespace-pre transition-all duration-500 invisible pointer-events-none dropdown overflow-hidden z-1"
    };

    let title_container_class = if title != "Price" {
        "flex gap-1 cursor-pointer justify-center items-center w-full"
    } else {
        "flex gap-1 justify-center items-center w-full"
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
            div id=(dropdown_id) class=(title_container_class) {
              span class="font-semibold" {(title)}
              @if title != "Price" {
                (drop_down_icon())
              }
            }
            div id=(format!("{}-items", dropdown_id)) class=(dropdown_items_class) {
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
            import {setupDropdown, setupPriceInput} from "/assets/js/app/searchbox.js";
            setupDropdown();
        </script>
      "#))
      div class="right-0 bottom-0 left-0 absolute flex justify-center items-center bg-[rgba(255,255,255,0.8)] p-6 h-60 flex-col gap-6" {
        div class="grid grid-cols-[300px_170px_170px_170px_200px_100px_100px] w-full max-w-7xl justify-center" {
          div class="flex justify-end items-center pr-3" {
            input class="rounded-md w-3/4 h-10 placeholder:text-sm" type="search" placeholder="Search Ref ID" ;
          }
          (render_search_box_selection("Listing Type", "/app/listing-type?demo=true", "listing-type-dropdown", "listing-type-label"))
          (render_search_box_selection("Location", "/rso/location?demo=true", "location-dropdown", "location-label"))
          (render_search_box_selection("Property Types", "/rso/property-types?demo=true", "property-types-dropdown", "property-types-label"))
          (render_search_box_selection("Price", "/app/prices", "price-dropdown", "price-label"))
          (render_search_box_selection("Bath", "/app/baths", "bath-dropdown", "bath-label"))
          (render_search_box_selection("Bed", "/app/beds", "bed-dropdown", "bed-label"))
        }
        div {
          button class="bg-blue-500 hover:bg-blue-700 text-white font-semibold py-3 px-14 rounded-md cursor-pointer" {
            "Search"
          }
        }
      }
    }
}
