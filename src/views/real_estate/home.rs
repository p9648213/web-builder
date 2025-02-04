use maud::{html, Markup, PreEscaped};
use tailwind_fuse::tw_merge;

use crate::{
    models::rso_data::{
        LocationDynamic, PropertyType, ProvinceAreaDynamic, SearchProperty, TextOrNum,
    },
    views::icons::{next_icon, previous_icon},
};

//.......................................................................................
//.BBBBBBBBB.......AAAA......NNNN....NNN..NNNN....NNN..EEEEEEEEEE..RRRRRRRRR.......1111..
//.BBBBBBBBBBB.....AAAAA.....NNNN....NNN..NNNN....NNN..EEEEEEEEEE..RRRRRRRRRRR.....1111..
//.BBBBBBBBBBB.....AAAAA.....NNNNN...NNN..NNNNN...NNN..EEEEEEEEEE..RRRRRRRRRRR...111111..
//.BBB.....BBB....AAAAAA.....NNNNN...NNN..NNNNN...NNN..EEE.........RRR.....RRR..1111111..
//.BBB.....BBB....AAAAAAA....NNNNNN..NNN..NNNNNN..NNN..EEE.........RRR.....RRR..1111111..
//.BBBBBBBBBB....AAAA.AAA....NNNNNNN.NNN..NNNNNNN.NNN..EEEEEEEEEE..RRRRRRRRRRR..11.1111..
//.BBBBBBBBBB....AAA..AAAA...NNN.NNN.NNN..NNN.NNN.NNN..EEEEEEEEEE..RRRRRRRRRR......1111..
//.BBBBBBBBBBB...AAAAAAAAA...NNN.NNNNNNN..NNN.NNNNNNN..EEEEEEEEEE..RRRRRRRR........1111..
//.BBB.....BBBB.AAAAAAAAAA...NNN..NNNNNN..NNN..NNNNNN..EEE.........RRR..RRRR.......1111..
//.BBB.....BBBB.AAAAAAAAAAA..NNN..NNNNNN..NNN..NNNNNN..EEE.........RRR...RRRR......1111..
//.BBBBBBBBBBB..AAA.....AAA..NNN...NNNNN..NNN...NNNNN..EEEEEEEEEEE.RRR....RRRR.....1111..
//.BBBBBBBBBBB.BAAA.....AAAA.NNN....NNNN..NNN....NNNN..EEEEEEEEEEE.RRR....RRRR.....1111..
//.BBBBBBBBBB..BAA......AAAA.NNN....NNNN..NNN....NNNN..EEEEEEEEEEE.RRR.....RRRR....1111..
//.......................................................................................

pub fn render_home_banner_1() -> Markup {
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
      (render_home_search_box_1())
    }
}

//.........................................................................................
//.BBBBBBBBB.......AAAA......NNNN....NNN..NNNN....NNN..EEEEEEEEEE..RRRRRRRRR.....222222....
//.BBBBBBBBBBB.....AAAAA.....NNNN....NNN..NNNN....NNN..EEEEEEEEEE..RRRRRRRRRRR..22222222...
//.BBBBBBBBBBB.....AAAAA.....NNNNN...NNN..NNNNN...NNN..EEEEEEEEEE..RRRRRRRRRRR..222..222...
//.BBB.....BBB....AAAAAA.....NNNNN...NNN..NNNNN...NNN..EEE.........RRR.....RRR.R222..2222..
//.BBB.....BBB....AAAAAAA....NNNNNN..NNN..NNNNNN..NNN..EEE.........RRR.....RRR.......2222..
//.BBBBBBBBBB....AAAA.AAA....NNNNNNN.NNN..NNNNNNN.NNN..EEEEEEEEEE..RRRRRRRRRRR.......222...
//.BBBBBBBBBB....AAA..AAAA...NNN.NNN.NNN..NNN.NNN.NNN..EEEEEEEEEE..RRRRRRRRRR.......2222...
//.BBBBBBBBBBB...AAAAAAAAA...NNN.NNNNNNN..NNN.NNNNNNN..EEEEEEEEEE..RRRRRRRR........2222....
//.BBB.....BBBB.AAAAAAAAAA...NNN..NNNNNN..NNN..NNNNNN..EEE.........RRR..RRRR......2222.....
//.BBB.....BBBB.AAAAAAAAAAA..NNN..NNNNNN..NNN..NNNNNN..EEE.........RRR...RRRR....2222......
//.BBBBBBBBBBB..AAA.....AAA..NNN...NNNNN..NNN...NNNNN..EEEEEEEEEEE.RRR....RRRR..2222.......
//.BBBBBBBBBBB.BAAA.....AAAA.NNN....NNNN..NNN....NNNN..EEEEEEEEEEE.RRR....RRRR.R222222222..
//.BBBBBBBBBB..BAA......AAAA.NNN....NNNN..NNN....NNNN..EEEEEEEEEEE.RRR.....RRRRR222222222..
//.........................................................................................

pub fn render_home_banner_2() -> Markup {
    html! {
      div class="flex justify-center items-center w-full" {
        div class="relative bg-[url('https://d1qawt2l8egll1.cloudfront.net/prod/images/240825202440-copy-of-moving-to-spain-from-the-uk.png')] bg-cover w-full h-screen" {
          div class="absolute inset-0 flex justify-center items-center m-auto max-w-360" {
            div class="flex justify-center items-center gap-15 mt-10 w-full" {
              div class="flex flex-col gap-5 bg-white opacity-80 p-8 rounded-lg max-w-md" {
                span class="font-bold text-4xl" { "Find your new home" }
                span class="text-lg" { "I'm a versatile paragraph. Add your own text and effortlessly customize me to make it your own. Feel free to edit and personalize your unique content!" }
              }
              (render_home_search_box_2())
            }
          }
        }
      }
    }
}

//.........................................................................................
//.BBBBBBBBB.......AAAA......NNNN....NNN..NNNN....NNN..EEEEEEEEEE..RRRRRRRRR.....33333.....
//.BBBBBBBBBBB.....AAAAA.....NNNN....NNN..NNNN....NNN..EEEEEEEEEE..RRRRRRRRRRR..3333333....
//.BBBBBBBBBBB.....AAAAA.....NNNNN...NNN..NNNNN...NNN..EEEEEEEEEE..RRRRRRRRRRR..333.3333...
//.BBB.....BBB....AAAAAA.....NNNNN...NNN..NNNNN...NNN..EEE.........RRR.....RRR..333..333...
//.BBB.....BBB....AAAAAAA....NNNNNN..NNN..NNNNNN..NNN..EEE.........RRR.....RRR......3333...
//.BBBBBBBBBB....AAAA.AAA....NNNNNNN.NNN..NNNNNNN.NNN..EEEEEEEEEE..RRRRRRRRRRR.....3333....
//.BBBBBBBBBB....AAA..AAAA...NNN.NNN.NNN..NNN.NNN.NNN..EEEEEEEEEE..RRRRRRRRRR......3333....
//.BBBBBBBBBBB...AAAAAAAAA...NNN.NNNNNNN..NNN.NNNNNNN..EEEEEEEEEE..RRRRRRRR..........333...
//.BBB.....BBBB.AAAAAAAAAA...NNN..NNNNNN..NNN..NNNNNN..EEE.........RRR..RRRR.........3333..
//.BBB.....BBBB.AAAAAAAAAAA..NNN..NNNNNN..NNN..NNNNNN..EEE.........RRR...RRRR..R333..3333..
//.BBBBBBBBBBB..AAA.....AAA..NNN...NNNNN..NNN...NNNNN..EEEEEEEEEEE.RRR....RRRR..333..333...
//.BBBBBBBBBBB.BAAA.....AAAA.NNN....NNNN..NNN....NNNN..EEEEEEEEEEE.RRR....RRRR..33333333...
//.BBBBBBBBBB..BAA......AAAA.NNN....NNNN..NNN....NNNN..EEEEEEEEEEE.RRR.....RRRR..33333.....
//.........................................................................................

pub fn render_home_banner_3() -> Markup {
    html! {
      div class="flex justify-center items-center w-full" {
        div class="relative bg-[url('https://d1qawt2l8egll1.cloudfront.net/prod/images/240825202440-copy-of-moving-to-spain-from-the-uk.png')] bg-cover w-full h-screen" {
          div class="absolute inset-0 flex justify-center items-center m-auto max-w-360" {
            div class="flex flex-col justify-center items-center gap-15 w-full -translate-y-35" {
              div class="flex flex-col gap-5 bg-white opacity-80 p-8 rounded-lg max-w-lg" {
                span class="font-bold text-4xl" { "Find your new home" }
                span class="text-lg" { "I'm a versatile paragraph. Add your own text and effortlessly customize me to make it your own. Feel free to edit and personalize your unique content!" }
              }
            }
          }
          (render_home_search_box_3())
        }
      }
    }
}

//.........................................................................................
//.BBBBBBBBB.......AAAA......NNNN....NNN..NNNN....NNN..EEEEEEEEEE..RRRRRRRRR........444....
//.BBBBBBBBBBB.....AAAAA.....NNNN....NNN..NNNN....NNN..EEEEEEEEEE..RRRRRRRRRRR.....4444....
//.BBBBBBBBBBB.....AAAAA.....NNNNN...NNN..NNNNN...NNN..EEEEEEEEEE..RRRRRRRRRRR.....4444....
//.BBB.....BBB....AAAAAA.....NNNNN...NNN..NNNNN...NNN..EEE.........RRR.....RRR....44444....
//.BBB.....BBB....AAAAAAA....NNNNNN..NNN..NNNNNN..NNN..EEE.........RRR.....RRR...444444....
//.BBBBBBBBBB....AAAA.AAA....NNNNNNN.NNN..NNNNNNN.NNN..EEEEEEEEEE..RRRRRRRRRRR...444444....
//.BBBBBBBBBB....AAA..AAAA...NNN.NNN.NNN..NNN.NNN.NNN..EEEEEEEEEE..RRRRRRRRRR...444.444....
//.BBBBBBBBBBB...AAAAAAAAA...NNN.NNNNNNN..NNN.NNNNNNN..EEEEEEEEEE..RRRRRRRR....R44..444....
//.BBB.....BBBB.AAAAAAAAAA...NNN..NNNNNN..NNN..NNNNNN..EEE.........RRR..RRRR...R444444444..
//.BBB.....BBBB.AAAAAAAAAAA..NNN..NNNNNN..NNN..NNNNNN..EEE.........RRR...RRRR..R444444444..
//.BBBBBBBBBBB..AAA.....AAA..NNN...NNNNN..NNN...NNNNN..EEEEEEEEEEE.RRR....RRRR......444....
//.BBBBBBBBBBB.BAAA.....AAAA.NNN....NNNN..NNN....NNNN..EEEEEEEEEEE.RRR....RRRR......444....
//.BBBBBBBBBB..BAA......AAAA.NNN....NNNN..NNN....NNNN..EEEEEEEEEEE.RRR.....RRRR.....444....
//.........................................................................................

pub fn render_home_banner_4() -> Markup {
    html! {
      div class="flex justify-center items-center w-full" {
        div class="relative bg-[url('https://d1qawt2l8egll1.cloudfront.net/prod/images/240825202440-copy-of-moving-to-spain-from-the-uk.png')] bg-cover w-full h-screen" {
          div class="absolute inset-0 flex justify-center items-center m-auto max-w-360" {
            div class="flex flex-col justify-center items-center gap-15 w-full -translate-y-35" {
              div class="flex flex-col gap-5 bg-white opacity-80 p-8 rounded-lg max-w-lg" {
                span class="font-bold text-4xl" { "Find your new home" }
                span class="text-lg" { "I'm a versatile paragraph. Add your own text and effortlessly customize me to make it your own. Feel free to edit and personalize your unique content!" }
              }
            }
          }
          (render_home_search_box_4())
        }
      }
    }
}

//.......................................................................................
//....SSSSSS....EEEEEEEEEE.....AAAA......RRRRRRRRR.......CCCCCC....HHH.....HHH.....1111..
//..SSSSSSSSS...EEEEEEEEEE.....AAAAA.....RRRRRRRRRRR...CCCCCCCCC...HHH.....HHH.....1111..
//..SSSSSSSSSS..EEEEEEEEEE.....AAAAA.....RRRRRRRRRRR..CCCCCCCCCCC..HHH.....HHH...111111..
//..SSS...SSSS..EEE...........AAAAAA.....RRR.....RRR..CCCC...CCCC..HHH.....HHH..1111111..
//..SSSS........EEE...........AAAAAAA....RRR.....RRR..CCC.....CC...HHH.....HHH..1111111..
//..SSSSSSS.....EEEEEEEEEE...AAAA.AAA....RRRRRRRRRRR.CCCC..........HHHHHHHHHHH..11.1111..
//...SSSSSSSS...EEEEEEEEEE...AAA..AAAA...RRRRRRRRRR..CCCC..........HHHHHHHHHHH.....1111..
//.....SSSSSSS..EEEEEEEEEE...AAAAAAAAA...RRRRRRRR....CCCC..........HHHHHHHHHHH.....1111..
//.........SSSS.EEE.........AAAAAAAAAA...RRR..RRRR....CCC.....CC...HHH.....HHH.....1111..
//.SSSS....SSSS.EEE.........AAAAAAAAAAA..RRR...RRRR...CCCC...CCCC..HHH.....HHH.....1111..
//..SSSSSSSSSS..EEEEEEEEEEE.AAA.....AAA..RRR....RRRR..CCCCCCCCCCC..HHH.....HHH.....1111..
//..SSSSSSSSSS..EEEEEEEEEEEAAAA.....AAAA.RRR....RRRR...CCCCCCCCC...HHH.....HHH.....1111..
//....SSSSSS....EEEEEEEEEEEAAA......AAAA.RRR.....RRRR....CCCCCC....HHH.....HHH.....1111..
//.......................................................................................

pub fn render_selection_drop_down_1(choices: Vec<&str>, highlight: &str) -> Markup {
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

pub fn render_beds_baths_selection_drop_down_1(choices: Vec<&str>, highlight: &str) -> Markup {
    html! {
      div class="flex flex-col justify-center gap-4 px-2 h-full text-sm" {
        div class="flex border-slate-900 border rounded-md h-10" {
          @for choice in choices {
            @if choice == highlight {
              div class="flex justify-center items-center border-slate-900 bg-blue-400 border-r last:border-none first:rounded-tl-md last:rounded-tr-md first:rounded-bl-md last:rounded-br-md w-10.5 text-white cursor-pointer" {
                (choice)
              }
            }@else {
              div class="flex justify-center items-center border-slate-900 hover:bg-blue-300 border-r last:border-none first:rounded-tl-md last:rounded-tr-md first:rounded-bl-md last:rounded-br-md w-10.5 hover:text-white cursor-pointer" {
                (choice)
              }
            }
          }
        }
        div class="flex items-center gap-2" {
          input id="exact" class="rounded-sm" name="exact-checkbox" type="checkbox";
          label for="exact" {"Exact number"}
        }
      }
    }
}

pub fn render_location_selection_drop_down_1(
    provinces: ProvinceAreaDynamic,
    highlight: &str,
) -> Markup {
    html! {
      div class="flex flex-col gap-3" {
        @match provinces {
          ProvinceAreaDynamic::Single(province_area) => {
            @if highlight == "All" {
              @let id = format!("{}-location", province_area.province_area_name);
              (render_input_radio_1("All", "province", "all-location", Some(true)))
              (render_input_radio_1(province_area.province_area_name.as_str(), "province", id.as_str(), None))

              @match province_area.locations.location {
                LocationDynamic::Single(location) => {
                  @let id = format!("{}-child", location);
                  (render_check_box_1(location.as_str(), None, id.as_str(), None, None));
                },
                LocationDynamic::Multiple(locations) => {
                  @for location in locations {
                    @let id = format!("{}-child", location);
                    (render_check_box_1(location.as_str(), None, id.as_str(), None, None));
                  }
                },
              }

            } @else {
              @let id = format!("{}-location", province_area.province_area_name);

              (render_input_radio_1("All", "province", "all-location", None))

              @if province_area.province_area_name == highlight {
                (render_input_radio_1(province_area.province_area_name.as_str(), "province", id.as_str(), Some(true)))
              }@else {
                (render_input_radio_1(province_area.province_area_name.as_str(), "province", id.as_str(), None))
              }
            }
          },
          ProvinceAreaDynamic::Multiple(province_areas) => {
            @if highlight == "All" {
              (render_input_radio_1("All", "province", "all-location", Some(true)))
            } @else {
              (render_input_radio_1("All", "province", "all-location", None))
            }

            @for province_area in province_areas {
              @let id = format!("{}-location", province_area.province_area_name);
              (render_input_radio_1(province_area.province_area_name.as_str(), "province", id.as_str(), None))

              @match province_area.locations.location {
                LocationDynamic::Single(location) => {
                  @let id = format!("{}-child", location);
                  (render_check_box_1(location.as_str(), None, id.as_str(), None, None));
                },
                LocationDynamic::Multiple(locations) => {
                  @for location in locations {
                    @let id = format!("{}-child", location);
                    (render_check_box_1(location.as_str(), None, id.as_str(), None, None));
                  }
                },
              }
            }
          },
        }
      }
    }
}

pub fn render_property_types_selection_drop_down_1(property_types: Vec<PropertyType>) -> Markup {
    html! {
      div class="flex flex-col gap-3" {
        @for property_type in property_types {
          (render_check_box_1(property_type.prop_type.as_str(), Some(property_type.option_value.as_str()) , property_type.option_value.as_str(), None, Some("ml-0")));
          @for sub_type in property_type.sub_types {
            (render_check_box_1(sub_type.prop_sub_type.as_str(), Some(property_type.option_value.as_str()), sub_type.sub_type_option_value.as_str(), None, None));
          }
        }
      }
    }
}

pub fn render_input_radio_1(value: &str, name: &str, id: &str, checked: Option<bool>) -> Markup {
    html! {
      div class="flex items-center gap-2 text-sm" {
        input type="radio" checked=[checked] id=(id) name=(name) value=(value);
        label for=(id) {(value)}
      }
    }
}

pub fn render_check_box_1(
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

pub fn render_selection_label_1(label: &str, id: &str) -> Markup {
    html! {
      div id=(id) hx-swap-oob="outerHTML" class="text-slate-500 text-sm" {
        (label)
      }
    }
}

pub fn render_price_input_1(id: &str) -> Markup {
    html! {
      (PreEscaped(r#"
        <script type="module">
            import {setupPriceInput} from "/assets/js/app/searchbox.js";
            setupPriceInput();
        </script>
      "#))
      div id=(id) hx-swap-oob="outerHTML" {
        label class="relative" {
          span class="top-[2px] right-0 absolute pr-2 text-slate-500 text-sm" {"€"}
          input id=(format!("{}-min", id)) placeholder="From" class="bg-transparent pr-4.5 border-t-0 border-r border-r-slate-700 border-b-0 border-l-0 focus:ring-0 w-[50%] h-5 text-sm placeholder:text-slate-500 placeholder:text-sm outline-none";
        }
        label class="relative" {
          span class="top-[2px] right-0 absolute pr-2 text-slate-500 text-sm" {"€"}
          input id=(format!("{}-max", id)) placeholder="To" class="bg-transparent pr-4.5 border-none focus:ring-0 w-[50%] h-5 text-sm placeholder:text-slate-500 placeholder:text-sm outline-none";
        }

      }
    }
}

pub fn render_search_box_selection_1(
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
        if title == "Bed" || title == "Bath" {
            "top-7 absolute flex flex-col gap-1 bg-white opacity-0 shadow p-2 rounded-md h-0 max-h-0 whitespace-pre transition-all duration-500 invisible pointer-events-none dropdown overflow-auto z-1 right-0"
        } else {
            "top-7 absolute flex flex-col gap-1 bg-white opacity-0 shadow p-2 rounded-md h-0 max-h-0 whitespace-pre transition-all duration-500 invisible pointer-events-none dropdown overflow-auto z-1"
        }
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
                img class="w-4 h-4" src="/assets/images/icon/dropdown.svg" alt="dropdown";
              }
            }
            div id=(format!("{}-items", dropdown_id)) class=(dropdown_items_class) {
              "Loading..."
            }
          }
          div id=(label_id) class="text-slate-500 text-sm" {
            "Loading..."
          }
        }
      }
    }
}

pub fn render_home_search_box_1() -> Markup {
    html! {
      (PreEscaped(r#"
        <script type="module">
            import {setupDropdown} from "/assets/js/app/searchbox.js";
            setupDropdown();
        </script>
      "#))
      div class="right-0 bottom-0 left-0 absolute flex flex-col justify-center items-center gap-6 bg-[rgba(255,255,255,0.8)] p-6 h-60" {
        div class="justify-center grid grid-cols-[300px_170px_170px_170px_200px_100px_100px] w-full max-w-7xl" {
          div class="flex justify-end items-center pr-3" {
            input class="border-slate-800 rounded-md w-3/4 h-10 placeholder:text-sm" type="search" placeholder="Search Ref ID" ;
          }
          (render_search_box_selection_1("Listing Type", "/data/real-estate/listing-type", "listing-type-dropdown", "listing-type-label"))
          (render_search_box_selection_1("Location", "/rso/location", "location-dropdown", "location-label"))
          (render_search_box_selection_1("Property Types", "/rso/property-types", "property-types-dropdown", "property-types-label"))
          (render_search_box_selection_1("Price", "/data/real-estate/prices", "price-dropdown", "price-label"))
          (render_search_box_selection_1("Bath", "/data/real-estate/baths", "bath-dropdown", "bath-label"))
          (render_search_box_selection_1("Bed", "/data/real-estate/beds", "bed-dropdown", "bed-label"))
        }
        div {
          button
            hx-get="/section/real-estate/contents/search-results"
            hx-push-url="/search-results"
            hx-target="main"
            class="bg-blue-500 hover:bg-blue-400 px-14 py-3 rounded-md font-semibold text-white cursor-pointer"
          {
            "Search"
          }
        }
      }
    }
}

//.........................................................................................
//....SSSSSS....EEEEEEEEEE.....AAAA......RRRRRRRRR.......CCCCCC....HHH.....HHH...222222....
//..SSSSSSSSS...EEEEEEEEEE.....AAAAA.....RRRRRRRRRRR...CCCCCCCCC...HHH.....HHH..22222222...
//..SSSSSSSSSS..EEEEEEEEEE.....AAAAA.....RRRRRRRRRRR..CCCCCCCCCCC..HHH.....HHH..222..222...
//..SSS...SSSS..EEE...........AAAAAA.....RRR.....RRR..CCCC...CCCC..HHH.....HHH.2222..2222..
//..SSSS........EEE...........AAAAAAA....RRR.....RRR..CCC.....CC...HHH.....HHH.......2222..
//..SSSSSSS.....EEEEEEEEEE...AAAA.AAA....RRRRRRRRRRR.CCCC..........HHHHHHHHHHH.......222...
//...SSSSSSSS...EEEEEEEEEE...AAA..AAAA...RRRRRRRRRR..CCCC..........HHHHHHHHHHH......2222...
//.....SSSSSSS..EEEEEEEEEE...AAAAAAAAA...RRRRRRRR....CCCC..........HHHHHHHHHHH.....2222....
//.........SSSS.EEE.........AAAAAAAAAA...RRR..RRRR....CCC.....CC...HHH.....HHH....2222.....
//.SSSS....SSSS.EEE.........AAAAAAAAAAA..RRR...RRRR...CCCC...CCCC..HHH.....HHH...2222......
//..SSSSSSSSSS..EEEEEEEEEEE.AAA.....AAA..RRR....RRRR..CCCCCCCCCCC..HHH.....HHH..2222.......
//..SSSSSSSSSS..EEEEEEEEEEEAAAA.....AAAA.RRR....RRRR...CCCCCCCCC...HHH.....HHH.2222222222..
//....SSSSSS....EEEEEEEEEEEAAA......AAAA.RRR.....RRRR....CCCCCC....HHH.....HHH.2222222222..
//.........................................................................................

pub fn render_home_search_box_2() -> Markup {
    html! {
      div class="bg-white p-7 rounded-lg w-110" {
        div class="flex flex-col gap-4" {
          div class="flex gap-6" {
            span class="pb-1 border-b-2 border-b-blue-500 font-bold" { "For Sales" }
            span class="pb-1" { "For Rent" }
          }
          div class="flex gap-6" {
            div class="flex items-center gap-2" {
              input type="radio" name="listing-type" checked value="sale";
              label { "ReSales" }
            }
            div class="flex items-center gap-2" {
              input type="radio" name="listing-type" value="new-dev";
              label { "New Dev" }
            }
          }
          div {
            input class="border-slate-800 rounded-md w-full h-10 placeholder:text-sm" type="search" placeholder="Search Ref ID" ;
          }
          div class="flex flex-col gap-3" {
            div class="flex flex-col gap-2" {
              span class="font-bold" { "Location" }
              div class="flex justify-between items-center border-slate-800 px-3 py-2 border rounded-md w-full" {
                span { "All" }
                img class="w-4 h-4" src="/assets/images/icon/dropdown.svg" alt="dropdown";
              }
            }
            div class="flex flex-col gap-2" {
              span class="font-bold" { "Property Types" }
              div class="flex justify-between items-center border-slate-800 px-3 py-2 border rounded-md w-full" {
                span { "All" }
                img class="w-4 h-4" src="/assets/images/icon/dropdown.svg" alt="dropdown";
              }
            }
            div class="flex gap-4" {
              div class="flex flex-col flex-1 gap-2" {
                span class="font-bold" { "Bed" }
                div class="flex justify-between items-center border-slate-800 px-3 py-2 border rounded-md w-full" {
                  div class="flex items-center gap-2" {
                    img class="w-5 h-5" src="/assets/images/icon/bed.svg" alt="bed";
                    span { "Any" }
                  }
                  div {
                    img class="w-4 h-4" src="/assets/images/icon/dropdown.svg" alt="dropdown";
                  }
                }
              }
              div class="flex flex-col flex-1 gap-2" {
                span class="font-bold" { "Bath" }
                div class="flex justify-between items-center border-slate-800 px-3 py-2 border rounded-md w-full" {
                  div class="flex items-center gap-2" {
                    img class="w-5 h-5" src="/assets/images/icon/bath.svg" alt="bath";
                    span { "Any" }
                  }
                  div {
                    img class="w-4 h-4" src="/assets/images/icon/dropdown.svg" alt="dropdown";
                  }
                }
              }
            }
            div class="flex flex-col gap-2" {
              span class="font-bold" { "Price" }
              div class="flex gap-4" {
                div class="flex flex-col flex-1 gap-2" {
                  div class="flex justify-between items-center border-slate-800 px-3 py-2 border rounded-md w-full" {
                    span { "Min €" }
                    div {
                      img class="w-4 h-4" src="/assets/images/icon/dropdown.svg" alt="dropdown";
                    }
                  }
                }
                div class="flex flex-col flex-1 gap-2" {
                  div class="flex justify-between items-center border-slate-800 px-3 py-2 border rounded-md w-full" {
                    span { "Max €" }
                    div {
                      img class="w-4 h-4" src="/assets/images/icon/dropdown.svg" alt="dropdown";
                    }
                  }
                }
              }
            }
            button
              hx-get="/section/real-estate/contents/search-results"
              hx-push-url="/search-results"
              hx-target="main"
              class="bg-blue-500 hover:bg-blue-400 mt-4 px-14 py-3 rounded-md w-full font-semibold text-white cursor-pointer"
            {
              "Search"
            }
          }
        }
      }
    }
}

//.........................................................................................
//....SSSSSS....EEEEEEEEEE.....AAAA......RRRRRRRRR.......CCCCCC....HHH.....HHH...33333.....
//..SSSSSSSSS...EEEEEEEEEE.....AAAAA.....RRRRRRRRRRR...CCCCCCCCC...HHH.....HHH..3333333....
//..SSSSSSSSSS..EEEEEEEEEE.....AAAAA.....RRRRRRRRRRR..CCCCCCCCCCC..HHH.....HHH..333.3333...
//..SSS...SSSS..EEE...........AAAAAA.....RRR.....RRR..CCCC...CCCC..HHH.....HHH..333..333...
//..SSSS........EEE...........AAAAAAA....RRR.....RRR..CCC.....CC...HHH.....HHH......3333...
//..SSSSSSS.....EEEEEEEEEE...AAAA.AAA....RRRRRRRRRRR.CCCC..........HHHHHHHHHHH.....3333....
//...SSSSSSSS...EEEEEEEEEE...AAA..AAAA...RRRRRRRRRR..CCCC..........HHHHHHHHHHH.....3333....
//.....SSSSSSS..EEEEEEEEEE...AAAAAAAAA...RRRRRRRR....CCCC..........HHHHHHHHHHH.......333...
//.........SSSS.EEE.........AAAAAAAAAA...RRR..RRRR....CCC.....CC...HHH.....HHH.......3333..
//.SSSS....SSSS.EEE.........AAAAAAAAAAA..RRR...RRRR...CCCC...CCCC..HHH.....HHH.3333..3333..
//..SSSSSSSSSS..EEEEEEEEEEE.AAA.....AAA..RRR....RRRR..CCCCCCCCCCC..HHH.....HHH..333..333...
//..SSSSSSSSSS..EEEEEEEEEEEAAAA.....AAAA.RRR....RRRR...CCCCCCCCC...HHH.....HHH..33333333...
//....SSSSSS....EEEEEEEEEEEAAA......AAAA.RRR.....RRRR....CCCCCC....HHH.....HHH...33333.....
//.........................................................................................

pub fn render_home_search_box_3() -> Markup {
    html! {
      div class="right-0 bottom-10 left-0 absolute flex justify-center" {
        div class="flex flex-col justify-center items-center gap-3 bg-white px-8 py-6 rounded-xl" {
          div class="flex gap-6" {
            div {
              input class="border-slate-800 rounded-md w-full h-12 placeholder:text-sm" type="search" placeholder="Search Ref ID" ;
            }
            div class="flex items-center gap-4 px-4 py-2 border border-black rounded-full" {
              "For Sale"
              img class="w-4 h-4" src="/assets/images/icon/dropdown.svg" alt="dropdown";
            }
            div class="flex items-center gap-4 px-4 py-2 border border-black rounded-full" {
              "All Locations"
              img class="w-4 h-4" src="/assets/images/icon/dropdown.svg" alt="dropdown";
            }
            div class="flex items-center gap-4 px-4 py-2 border border-black rounded-full" {
              "All Property Types"
              img class="w-4 h-4" src="/assets/images/icon/dropdown.svg" alt="dropdown";
            }
            div class="flex items-center gap-4 px-4 py-2 border border-black rounded-full" {
              "Price"
              img class="w-4 h-4" src="/assets/images/icon/dropdown.svg" alt="dropdown";
            }
            div class="flex items-center gap-4 px-4 py-2 border border-black rounded-full" {
              "Bed"
              img class="w-4 h-4" src="/assets/images/icon/dropdown.svg" alt="dropdown";
            }
            div class="flex items-center gap-4 px-4 py-2 border border-black rounded-full" {
              "Bath"
              img class="w-4 h-4" src="/assets/images/icon/dropdown.svg" alt="dropdown";
            }
          }
          button
            hx-get="/section/real-estate/contents/search-results"
            hx-push-url="/search-results"
            hx-target="main"
            class="bg-blue-500 hover:bg-blue-400 mt-4 px-14 py-3 rounded-md w-fit font-semibold text-white cursor-pointer"
          {
            "Search"
          }
        }
      }
    }
}

//.........................................................................................
//....SSSSSS....EEEEEEEEEE.....AAAA......RRRRRRRRR.......CCCCCC....HHH.....HHH......444....
//..SSSSSSSSS...EEEEEEEEEE.....AAAAA.....RRRRRRRRRRR...CCCCCCCCC...HHH.....HHH.....4444....
//..SSSSSSSSSS..EEEEEEEEEE.....AAAAA.....RRRRRRRRRRR..CCCCCCCCCCC..HHH.....HHH.....4444....
//..SSS...SSSS..EEE...........AAAAAA.....RRR.....RRR..CCCC...CCCC..HHH.....HHH....44444....
//..SSSS........EEE...........AAAAAAA....RRR.....RRR..CCC.....CC...HHH.....HHH...444444....
//..SSSSSSS.....EEEEEEEEEE...AAAA.AAA....RRRRRRRRRRR.CCCC..........HHHHHHHHHHH...444444....
//...SSSSSSSS...EEEEEEEEEE...AAA..AAAA...RRRRRRRRRR..CCCC..........HHHHHHHHHHH..444.444....
//.....SSSSSSS..EEEEEEEEEE...AAAAAAAAA...RRRRRRRR....CCCC..........HHHHHHHHHHH.444..444....
//.........SSSS.EEE.........AAAAAAAAAA...RRR..RRRR....CCC.....CC...HHH.....HHH.4444444444..
//.SSSS....SSSS.EEE.........AAAAAAAAAAA..RRR...RRRR...CCCC...CCCC..HHH.....HHH.4444444444..
//..SSSSSSSSSS..EEEEEEEEEEE.AAA.....AAA..RRR....RRRR..CCCCCCCCCCC..HHH.....HHH......444....
//..SSSSSSSSSS..EEEEEEEEEEEAAAA.....AAAA.RRR....RRRR...CCCCCCCCC...HHH.....HHH......444....
//....SSSSSS....EEEEEEEEEEEAAA......AAAA.RRR.....RRRR....CCCCCC....HHH.....HHH......444....
//.........................................................................................

pub fn render_home_search_box_4() -> Markup {
    html! {
      div class="right-0 bottom-10 left-0 absolute flex justify-center" {
        div class="justify-center items-center gap-3 grid grid-cols-8 grid-rows-2 bg-white px-8 py-6 rounded-xl" {
          div class="flex justify-between items-center gap-4 col-span-2 px-4 py-2 border border-black rounded-md" {
            "For Sale"
            img class="w-4 h-4" src="/assets/images/icon/dropdown.svg" alt="dropdown";
          }
          div class="flex justify-between items-center gap-4 col-span-2 px-4 py-2 border border-black rounded-md" {
            div class="flex items-center gap-2" {
              img class="w-6 h-6" src="/assets/images/icon/location.svg" alt="location";
              "All Locations"
            }
            img class="w-4 h-4" src="/assets/images/icon/dropdown.svg" alt="dropdown";
          }
          div class="flex justify-between items-center gap-4 col-span-2 px-4 py-2 border border-black rounded-md" {
            "All Property Types"
            img class="w-4 h-4" src="/assets/images/icon/dropdown.svg" alt="dropdown";
          }
          div class="flex justify-between items-center gap-4 px-4 py-2 border border-black rounded-md" {
            "Min €"
            img class="w-4 h-4" src="/assets/images/icon/dropdown.svg" alt="dropdown";
          }
          div class="flex justify-between items-center gap-4 px-4 py-2 border border-black rounded-md" {
            "Max €"
            img class="w-4 h-4" src="/assets/images/icon/dropdown.svg" alt="dropdown";
          }
          div class="flex justify-between items-center gap-4 col-span-2 px-4 py-2 border border-black rounded-md" {
            div class="flex items-center gap-2" {
              img class="w-5 h-5" src="/assets/images/icon/bed.svg" alt="bed";
              "Any"
            }
            img class="w-4 h-4" src="/assets/images/icon/dropdown.svg" alt="dropdown";
          }
          div class="flex justify-between items-center gap-4 col-span-2 px-4 py-2 border border-black rounded-md" {
            div class="flex items-center gap-2" {
              img class="w-5 h-5" src="/assets/images/icon/bath.svg" alt="bath";
              "Any"
            }
            img class="w-4 h-4" src="/assets/images/icon/dropdown.svg" alt="dropdown";
          }
          div class="col-span-2" {
            input class="border-slate-800 rounded-md w-full h-10.5 placeholder:text-sm" type="search" placeholder="Search Ref ID";
          }
          button
            hx-get="/section/real-estate/contents/search-results"
            hx-push-url="/search-results"
            hx-target="main"
            class="col-span-2 bg-blue-500 hover:bg-blue-400 rounded-md w-full h-10.5 font-semibold text-white cursor-pointer"
          {
            "Search"
          }
        }
      }
    }
}

//................................................
//.HHH.....HHH.....OOOOOO.....TTTTTTTTTTT...1111..
//.HHH.....HHH...OOOOOOOOOO...TTTTTTTTTTT...1111..
//.HHH.....HHH..OOOOOOOOOOOO..TTTTTTTTTTT.111111..
//.HHH.....HHH..OOOO....OOOO......TTT....1111111..
//.HHH.....HHH..OOO......OOO......TTT....1111111..
//.HHHHHHHHHHH.HOOO......OOOO.....TTT....11.1111..
//.HHHHHHHHHHH.HOOO......OOOO.....TTT.......1111..
//.HHHHHHHHHHH.HOOO......OOOO.....TTT.......1111..
//.HHH.....HHH..OOO......OOO......TTT.......1111..
//.HHH.....HHH..OOOO....OOOO......TTT.......1111..
//.HHH.....HHH..OOOOOOOOOOOO......TTT.......1111..
//.HHH.....HHH...OOOOOOOOOO.......TTT.......1111..
//.HHH.....HHH.....OOOOOO.........TTT.......1111..
//................................................

pub fn render_hot_properties_1() -> Markup {
    html! {
      div class="flex justify-center items-center" {
        div class="flex justify-center px-15 py-20 w-full max-w-360" {
          div class="flex flex-col gap-12 w-full" {
            div class="flex flex-col gap-4" {
              h3 class="font-bold text-2xl" {
                "View our featured listings"
              }
              p class="max-w-120 text-xl" {
                "I'm a versatile paragraph. Add your own text and effortlessly customize me to make it your own. Feel free to edit and personalize your unique content!"
              }
              div class="border-b-3 border-b-black border-b-solid w-8" {}
            }
            div
              hx-get="/rso/hot-properties?theme=1"
              hx-trigger="load"
              hx-swap="innerHTML"
              class="flex flex-col justify-center items-center"
            {
              "Loading..."
            }
          }
        }
      }
    }
}

pub fn render_hot_properties_slider_1(hot_properties: Vec<SearchProperty>) -> Markup {
    let chunk_size = 6;
    let total_pages = (hot_properties.len() as f64 / chunk_size as f64).ceil();

    let mut properties_chunks = vec![];

    let mut i = 0;
    let mut j;

    while i < hot_properties.len() {
        let mut chunk = vec![];
        j = i;
        while j < i + chunk_size && j < hot_properties.len() {
            chunk.push(&hot_properties[j]);
            j = j + 1;
        }
        properties_chunks.push(chunk);
        i = i + chunk_size;
    }

    html! {
      (PreEscaped(r#"
        <script type="module">
            import {setupHotPropertySlider, setupPropertyPictureSlider} from "/assets/js/app/slider.js";
            setupHotPropertySlider();
            setupPropertyPictureSlider();
        </script>
      "#))
      input id="hot-props-total-pages" type="hidden" value=(total_pages) ;
      div class="flex justify-end gap-4 mb-4 w-full" {
        button
          id="hot-props-previous-button"
          class="border-slate-600 hover:bg-blue-400 p-2 border border-solid rounded-full cursor-pointer hover:stroke-white stroke-black"
        {
          (previous_icon())
        }
        button
          id="hot-props-next-button"
          class="border-slate-600 hover:bg-blue-400 p-2 border border-solid rounded-full cursor-pointer hover:stroke-white stroke-black"
        {
          (next_icon())
        }
      }
      div class="py-3 max-w-5xl overflow-x-hidden" {
        div id="hot-properties-slider" class="flex gap-10 transition-transform duration-500" {
          @for property_chunk in &properties_chunks {
            div class="gap-10 grid grid-cols-[292px_292px_292px] pl-12" {
              @for property in property_chunk {
                (render_hot_property_card_1(property, "sale"))
              }
            }
          }
        }
      }
      div class="flex justify-center gap-2 mt-4 w-full" id="hot-property-dots" {
        @for i in 0..total_pages as u8 {
          @if i == 0 {
            div class="bg-blue-500 p-1 rounded-full cursor-pointer" {}
          } @else {
            div class="bg-blue-200 p-1 rounded-full cursor-pointer" {}
          }
        }
      }
    }
}

pub fn render_hot_property_card_1(property: &SearchProperty, listing_type: &str) -> Markup {
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

    html! {
      div class="relative flex flex-col gap-2 shadow-md rounded-lg overflow-hidden picture-container" {
        div class="relative picture-slider-container" {
          div class="flex h-42 transition-transform duration-500 picture-slider" {
            input type="hidden" value=(total_pictures);
            (render_main_image)
            (render_images)
          }
          div class="bottom-2 left-[50%] absolute flex gap-2 max-w-18 -translate-x-[50%] overflow-hidden pictures-dots" {
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
          div class="font-bold" {
            @if property.newdev_name == "" {
              (property.property_type.name_type)
            }@else {
              (property.newdev_name)
            }
          }
          div class="flex flex-col gap-2" {
            div class="font-bold text-blue-500 text-lg" {
              (property.price) " €"
            }
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

//..................................................
//.HHH.....HHH.....OOOOOO.....TTTTTTTTTTT.222222....
//.HHH.....HHH...OOOOOOOOOO...TTTTTTTTTTT22222222...
//.HHH.....HHH..OOOOOOOOOOOO..TTTTTTTTTTT222..222...
//.HHH.....HHH..OOOO....OOOO......TTT...T222..2222..
//.HHH.....HHH..OOO......OOO......TTT.........2222..
//.HHHHHHHHHHH.HOOO......OOOO.....TTT.........222...
//.HHHHHHHHHHH.HOOO......OOOO.....TTT........2222...
//.HHHHHHHHHHH.HOOO......OOOO.....TTT.......2222....
//.HHH.....HHH..OOO......OOO......TTT......2222.....
//.HHH.....HHH..OOOO....OOOO......TTT.....2222......
//.HHH.....HHH..OOOOOOOOOOOO......TTT....2222.......
//.HHH.....HHH...OOOOOOOOOO.......TTT...T222222222..
//.HHH.....HHH.....OOOOOO.........TTT...T222222222..
//..................................................

pub fn render_hot_properties_2() -> Markup {
    html! {
      div class="flex justify-center items-center" {
        div class="flex justify-center px-15 py-20 w-full max-w-360" {
          div class="flex flex-col gap-12 w-full" {
            div class="flex flex-col gap-4" {
              h3 class="font-bold text-2xl" {
                "View our featured listings"
              }
              p class="max-w-190 text-[#868D9B] text-xl" {
                "I'm a versatile paragraph. Add your own text and effortlessly customize me to make it your own. Feel free to edit and personalize your unique content!"
              }
              div class="border-b-3 border-b-black border-b-solid w-8" {}
            }
            div
              hx-get="/rso/hot-properties?theme=2"
              hx-trigger="load"
              hx-swap="innerHTML"
              class="flex flex-col justify-center items-center"
            {
              "Loading..."
            }
          }
        }
      }
    }
}

pub fn render_hot_properties_slider_2(hot_properties: Vec<SearchProperty>) -> Markup {
    html! {
      div class="gap-3 grid grid-cols-3 grid-rows-2"  {
        @for (index, property) in hot_properties.iter().enumerate() {
          @if index < 5 {
            @if property.main_image.is_some() && index == 0 {
              div
                hx-get=(format!("/section/real-estate/contents/property?id={}&type={}", property.reference, "sale"))
                hx-push-url=(format!("/property?id={}&type={}", property.reference, "sale"))
                hx-trigger="click"
                hx-target="main"
                class="relative row-span-full rounded-lg cursor-pointer overflow-hidden"
              {
                img class="w-full h-full" src=(property.main_image.as_ref().unwrap()) alt="main-image";
                (render_infomation_box_2(&property, index))
              }
            } @else {
              @if property.pictures.is_some() {
                @if index == 0 {
                  div
                    hx-get=(format!("/section/real-estate/contents/property?id={}&type={}", property.reference, "sale"))
                    hx-push-url=(format!("/property?id={}&type={}", property.reference, "sale"))
                    hx-trigger="click"
                    hx-target="main"
                    class="relative row-span-full rounded-lg cursor-pointer overflow-hidden"
                  {
                    img class="w-full h-full" src=(property.pictures.as_ref().unwrap().picture[0].picture_url);
                    (render_infomation_box_2(&property, index))
                  }
                } @else {
                  div
                    hx-get=(format!("/section/real-estate/contents/property?id={}&type={}", property.reference, "sale"))
                    hx-push-url=(format!("/property?id={}&type={}", property.reference, "sale"))
                    hx-trigger="click"
                    hx-target="main"
                    class="relative rounded-lg cursor-pointer overflow-hidden"
                  {
                    img class="w-full h-full" src=(property.pictures.as_ref().unwrap().picture[0].picture_url);
                    (render_infomation_box_2(&property, index))
                  }
                }
              }
            }
          }
        }
      }
    }
}

pub fn render_infomation_box_2(property: &SearchProperty, index: usize) -> Markup {
    let built_size = match &property.built {
        TextOrNum::Text(built) => built.to_string(),
        TextOrNum::Num(built) => built.to_string(),
    };

    html! {
      div
        class="right-0 bottom-0 left-0 absolute flex flex-col gap-3 p-5 text-white"
        style="background-color: rgba(0, 0, 0, 0.4);"
      {
        div class="font-bold" {
          @if property.newdev_name == "" {
            (property.property_type.name_type)
          }@else {
            (property.newdev_name)
          }
        }
        @if index == 0 {
          div class="flex items-center gap-2" {
            img class="w-6 h-6" src="/assets/images/icon/location-white.svg" alt="location" ;
            span { (property.location) }
          }
        }
        div class="flex items-center gap-4" {
          div class="flex items-center gap-2" {
            img class="w-6 h-6" src="/assets/images/icon/bed-white.svg" alt="beds" ;
            span { (property.bedrooms) }
          }
          div class="flex items-center gap-2" {
            img class="w-6 h-6" src="/assets/images/icon/bath-white.svg" alt="baths" ;
            span { (property.bathrooms) }
          }
          div class="flex items-center gap-2" {
            img class="w-6 h-6" src="/assets/images/icon/built-size-white.svg" alt="built-size" ;
            span { (built_size) "m²" }
          }
          @if index > 0 {
            div class="flex items-center gap-2" {
              img class="w-6 h-6" src="/assets/images/icon/location-white.svg" alt="location" ;
              span { (property.location) }
            }
          }
        }
      }
    }
}

//..................................................
//.HHH.....HHH.....OOOOOO.....TTTTTTTTTTT.33333.....
//.HHH.....HHH...OOOOOOOOOO...TTTTTTTTTTT3333333....
//.HHH.....HHH..OOOOOOOOOOOO..TTTTTTTTTTT333.3333...
//.HHH.....HHH..OOOO....OOOO......TTT....333..333...
//.HHH.....HHH..OOO......OOO......TTT........3333...
//.HHHHHHHHHHH.HOOO......OOOO.....TTT.......3333....
//.HHHHHHHHHHH.HOOO......OOOO.....TTT.......3333....
//.HHHHHHHHHHH.HOOO......OOOO.....TTT.........333...
//.HHH.....HHH..OOO......OOO......TTT.........3333..
//.HHH.....HHH..OOOO....OOOO......TTT...T333..3333..
//.HHH.....HHH..OOOOOOOOOOOO......TTT....333..333...
//.HHH.....HHH...OOOOOOOOOO.......TTT....33333333...
//.HHH.....HHH.....OOOOOO.........TTT.....33333.....
//..................................................

pub fn render_hot_properties_3() -> Markup {
    html! {
      div class="flex justify-center items-center" {
        div class="flex justify-center px-15 py-20 w-full max-w-360" {
          div class="flex flex-col gap-12 w-full" {
            div class="flex flex-col gap-4" {
              h3 class="font-bold text-2xl" {
                "View our featured listings"
              }
              p class="max-w-120 text-xl" {
                "I'm a versatile paragraph. Add your own text and effortlessly customize me to make it your own. Feel free to edit and personalize your unique content!"
              }
              div class="border-b-3 border-b-black border-b-solid w-8" {}
            }
            div
              hx-get="/rso/hot-properties?theme=3"
              hx-trigger="load"
              hx-swap="innerHTML"
              class="flex flex-col justify-center items-center"
            {
              "Loading..."
            }
          }
        }
      }
    }
}

pub fn render_hot_properties_slider_3(hot_properties: Vec<SearchProperty>) -> Markup {
    html! {
      "Hot 3"
    }
}

//..................................................
//.HHH.....HHH.....OOOOOO.....TTTTTTTTTTT....444....
//.HHH.....HHH...OOOOOOOOOO...TTTTTTTTTTT...4444....
//.HHH.....HHH..OOOOOOOOOOOO..TTTTTTTTTTT...4444....
//.HHH.....HHH..OOOO....OOOO......TTT......44444....
//.HHH.....HHH..OOO......OOO......TTT.....444444....
//.HHHHHHHHHHH.HOOO......OOOO.....TTT.....444444....
//.HHHHHHHHHHH.HOOO......OOOO.....TTT....444.444....
//.HHHHHHHHHHH.HOOO......OOOO.....TTT...T44..444....
//.HHH.....HHH..OOO......OOO......TTT...T444444444..
//.HHH.....HHH..OOOO....OOOO......TTT...T444444444..
//.HHH.....HHH..OOOOOOOOOOOO......TTT........444....
//.HHH.....HHH...OOOOOOOOOO.......TTT........444....
//.HHH.....HHH.....OOOOOO.........TTT........444....
//..................................................

pub fn render_hot_properties_4() -> Markup {
    html! {
      div class="flex justify-center items-center" {
        div class="flex justify-center px-15 py-20 w-full max-w-360" {
          div class="flex flex-col gap-12 w-full" {
            div class="flex flex-col gap-4" {
              h3 class="font-bold text-2xl" {
                "View our featured listings"
              }
              p class="max-w-120 text-xl" {
                "I'm a versatile paragraph. Add your own text and effortlessly customize me to make it your own. Feel free to edit and personalize your unique content!"
              }
              div class="border-b-3 border-b-black border-b-solid w-8" {}
            }
            div
              hx-get="/rso/hot-properties?theme=4"
              hx-trigger="load"
              hx-swap="innerHTML"
              class="flex flex-col justify-center items-center"
            {
              "Loading..."
            }
          }
        }
      }
    }
}

pub fn render_hot_properties_slider_4(hot_properties: Vec<SearchProperty>) -> Markup {
    html! {
      "Hot 4"
    }
}

//.........................................................................................
//....SSSSSS....EEEEEEEEEE..RRRRRRRRR..RVVV.....VVVV.III.....CCCCC.....EEEEEEEEEE.....111..
//..SSSSSSSSS...EEEEEEEEEE..RRRRRRRRRRR.VVV.....VVVV.III...CCCCCCCCC...EEEEEEEEEE....1111..
//..SSSSSSSSSS..EEEEEEEEEE..RRRRRRRRRRR.VVVV....VVV..III..CCCCCCCCCCC..EEEEEEEEEE...11111..
//..SSS...SSSS..EEE.........RRR.....RRR.VVVV...VVVV..III..CCCC...CCCC..EEE.........111111..
//..SSSS........EEE.........RRR.....RRR..VVV...VVVV..III.CCCC.....CC...EEE.........111111..
//..SSSSSSS.....EEEEEEEEEE..RRRRRRRRRRR..VVVV..VVV...III.CCCC..........EEEEEEEEEE..11.111..
//...SSSSSSSS...EEEEEEEEEE..RRRRRRRRRR...VVVV.VVVV...III.CCCC..........EEEEEEEEEE.....111..
//.....SSSSSSS..EEEEEEEEEE..RRRRRRRR......VVV.VVVV...III.CCCC..........EEEEEEEEEE.....111..
//.........SSSS.EEE.........RRR..RRRR.....VVVVVVV....III.CCCC.....CC...EEE............111..
//.SSSS....SSSS.EEE.........RRR...RRRR....VVVVVVV....III..CCCC...CCCC..EEE............111..
//..SSSSSSSSSS..EEEEEEEEEEE.RRR....RRRR....VVVVV.....III..CCCCCCCCCCC..EEEEEEEEEE.....111..
//..SSSSSSSSSS..EEEEEEEEEEE.RRR....RRRR....VVVVV.....III...CCCCCCCCC...EEEEEEEEEE.....111..
//....SSSSSS....EEEEEEEEEEE.RRR.....RRRR...VVVVV.....III.....CCCCCC....EEEEEEEEEE.....111..
//.........................................................................................

pub fn render_our_services() -> Markup {
    html! {
      div class="flex justify-center items-center bg-slate-950 text-white" {
        div class="flex justify-center items-center gap-20 px-15 py-20 max-w-7xl" {
          div class="flex flex-col flex-1 gap-5" {
            (render_services_box("Find your property", "I'm a versatile paragraph. Add your own text and effortlessly customize me to make it your own. Feel free to edit and personalize your unique content!"))
            (render_services_box("Find your property", "I'm a versatile paragraph. Add your own text and effortlessly customize me to make it your own. Feel free to edit and personalize your unique content!"))
            (render_services_box("Find your property", "I'm a versatile paragraph. Add your own text and effortlessly customize me to make it your own. Feel free to edit and personalize your unique content!"))
          }
          div class="flex-1" {
            (render_services_descriptions("Short highlight of your services", "I'm a versatile paragraph. Add your own text and effortlessly customize me to make it your own. Feel free to edit and personalize your unique content!"))
          }
        }
      }
    }
}

pub fn render_services_box(title: &str, description: &str) -> Markup {
    html! {
      div class="bg-slate-500 px-5 p-3 rounded-lg" {
        div class="flex items-center gap-4" {
          div class="flex flex-col gap-2 text-lg" {
            div class="text-slate-950" {
              (title)
            }
            div {
              (description)
            }
          }
          div {
            img class="w-20 h-20" alt="check" src="/assets/images/icon/check.svg";
          }
        }
      }
    }
}

pub fn render_services_descriptions(title: &str, description: &str) -> Markup {
    html! {
      div class="flex flex-col gap-4" {
        div class="flex flex-col gap-2 text-3xl"{
          (title)
          div class="border-t-2 border-t-white w-7" {}
        }
        div class="text-lg"{
          (description)
        }
      }
    }
}

//.............................................................
//.TTTTTTTTTTTEEEEEEEEEE....SSSSSS....TTTTTTTTTTTIII.....1111..
//.TTTTTTTTTTTEEEEEEEEEE..SSSSSSSSS...TTTTTTTTTTTIII.....1111..
//.TTTTTTTTTTTEEEEEEEEEE..SSSSSSSSSS..TTTTTTTTTTTIII...111111..
//.....TTT....EEE.........SSS...SSSS......TTT....III..1111111..
//.....TTT....EEE.........SSSS............TTT....III..1111111..
//.....TTT....EEEEEEEEEE..SSSSSSS.........TTT....III..11.1111..
//.....TTT....EEEEEEEEEE...SSSSSSSS.......TTT....III.....1111..
//.....TTT....EEEEEEEEEE.....SSSSSSS......TTT....III.....1111..
//.....TTT....EEE................SSSS.....TTT....III.....1111..
//.....TTT....EEE........ESSS....SSSS.....TTT....III.....1111..
//.....TTT....EEEEEEEEEEE.SSSSSSSSSS......TTT....III.....1111..
//.....TTT....EEEEEEEEEEE.SSSSSSSSSS......TTT....III.....1111..
//.....TTT....EEEEEEEEEEE...SSSSSS........TTT....III.....1111..
//.............................................................

pub fn render_testimonial() -> Markup {
    html! {
      div class="flex justify-center items-center" {
        div class="flex flex-col justify-center items-center gap-20 px-15 py-20 max-w-360" {
          div class="flex flex-col justify-center gap-10 w-full font-bold text-center" {
            div class="text-lg" {
              "Testimonials"
            }
            div class="text-xl" {
              "What do our customers think about us?"
            }
          }
          div class="p-3 max-w-6xl overflow-hidden" {
            div class="flex gap-12" {
              @for _ in 0..6 {
                ((render_testimonial_card()))
              }
            }
          }
        }
      }
    }
}

pub fn render_testimonial_card() -> Markup {
    html! {
      div class="flex flex-col gap-10 p-7 rounded-lg w-84 shrink-0" style="box-shadow: rgba(67, 71, 85, 0.27) 0px 0px 0.25em, rgba(90, 125, 188, 0.05) 0px 0.25em 1em;" {
        div class="flex flex-col justify-center items-center gap-4" {
          div class="shadow-md rounded-full w-17 h-17 overflow-hidden" {
            img class="w-full" src="https://d1qawt2l8egll1.cloudfront.net/prod/images/231117083301-51-22.jpg" ;
          }

          div class="text-sm" {
            "Linda"
          }

          div class="flex gap-2" {
            @for _ in 0..5 {
              img class="w-6 h-6" alt="star" src="/assets/images/icon/star.svg";
            }
          }

          div class="text-center text-neutral-500" {
            "Moving to Spain was a big decision to me, especially finding a house. Then I found DEMO AGENCY and my journey from that moment was much easier than before. Thanks for their wise knowledge and professional skills."
          }
        }

        div class="text-right text-sm" {
          "- I got an amazing new home -"
        }
      }
    }
}
