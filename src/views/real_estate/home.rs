use maud::{html, Markup, PreEscaped};
use tailwind_fuse::tw_merge;

use crate::{
    models::rso_data::{
        LocationDynamic, PropertyType, ProvinceAreaDynamic, SearchProperty, TextOrNum,
    },
    views::icons::{
        next_icon, previous_icon
    },
};

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

pub fn render_home_search_box() -> Markup {
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
          (render_search_box_selection("Listing Type", "/data/real-estate/listing-type", "listing-type-dropdown", "listing-type-label"))
          (render_search_box_selection("Location", "/rso/location", "location-dropdown", "location-label"))
          (render_search_box_selection("Property Types", "/rso/property-types", "property-types-dropdown", "property-types-label"))
          (render_search_box_selection("Price", "/data/real-estate/prices", "price-dropdown", "price-label"))
          (render_search_box_selection("Bath", "/data/real-estate/baths", "bath-dropdown", "bath-label"))
          (render_search_box_selection("Bed", "/data/real-estate/beds", "bed-dropdown", "bed-label"))
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

//......................................................................................................
//.HHH.....HHH.....OOOOOO.....TTTTTTTTTTTPPPPPPPPP...RRRRRRRRR.......OOOOOO.....PPPPPPPPP.....SSSSSS....
//.HHH.....HHH...OOOOOOOOOO...TTTTTTTTTTTPPPPPPPPPP..RRRRRRRRRRR...OOOOOOOOOO...PPPPPPPPPP..SSSSSSSSS...
//.HHH.....HHH..OOOOOOOOOOOO..TTTTTTTTTTTPPPPPPPPPP..RRRRRRRRRRR..OOOOOOOOOOOO..PPPPPPPPPP..SSSSSSSSSS..
//.HHH.....HHH..OOOO....OOOO......TTT....PPP....PPPP.RRR.....RRR..OOOO....OOOO..PPP....PPPP.SSS...SSSS..
//.HHH.....HHH..OOO......OOO......TTT....PPP....PPPP.RRR.....RRR..OOO......OOO..PPP....PPPP.SSSS........
//.HHHHHHHHHHH.HOOO......OOOO.....TTT....PPPPPPPPPP..RRRRRRRRRRR.ROOO......OOOO.PPPPPPPPPP..SSSSSSS.....
//.HHHHHHHHHHH.HOOO......OOOO.....TTT....PPPPPPPPPP..RRRRRRRRRR..ROOO......OOOO.PPPPPPPPPP...SSSSSSSS...
//.HHHHHHHHHHH.HOOO......OOOO.....TTT....PPPPPPPPP...RRRRRRRR....ROOO......OOOO.PPPPPPPPP......SSSSSSS..
//.HHH.....HHH..OOO......OOO......TTT....PPP.........RRR..RRRR....OOO......OOO..PPP................SSS..
//.HHH.....HHH..OOOO....OOOO......TTT....PPP.........RRR...RRRR...OOOO....OOOO..PPP........PSSS....SSS..
//.HHH.....HHH..OOOOOOOOOOOO......TTT....PPP.........RRR....RRRR..OOOOOOOOOOOO..PPP.........SSSSSSSSSS..
//.HHH.....HHH...OOOOOOOOOO.......TTT....PPP.........RRR....RRRR...OOOOOOOOOO...PPP.........SSSSSSSSSS..
//.HHH.....HHH.....OOOOOO.........TTT....PPP.........RRR.....RRRR....OOOOOO.....PPP...........SSSSSS....
//......................................................................................................

pub fn render_hot_properties() -> Markup {
    html! {
      div class="flex justify-center items-center" {
        div class="flex justify-center px-15 py-20 w-full" {
          div class="flex flex-col gap-12" {
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
              hx-get="/rso/hot-properties"
              hx-trigger="load"
              hx-swap="innerHTML"
            {
              "Loading..."
            }
          }
        }
      }
    }
}

pub fn render_hot_properties_slider(hot_properties: Vec<SearchProperty>) -> Markup {
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
                (render_hot_property_card(property, "sale"))
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

pub fn render_hot_property_card(property: &SearchProperty, listing_type: &str) -> Markup {
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
                img class="h-5 w-5" alt="bed" src="/assets/images/icon/bed.svg";
                (property.bedrooms)
              }
              div class="flex items-center gap-2" {
                img class="h-5 w-5" alt="bath" src="/assets/images/icon/bath.svg";
                (property.bathrooms)
              }
              div class="flex items-center gap-2" {
                img class="h-5 w-5" alt="buit size" src="/assets/images/icon/built-size.svg";
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

//.................................................................................
//....SSSSSS....EEEEEEEEEE..RRRRRRRRR..RVVV.....VVVV.III.....CCCCC.....EEEEEEEEEE..
//..SSSSSSSSS...EEEEEEEEEE..RRRRRRRRRRR.VVV.....VVVV.III...CCCCCCCCC...EEEEEEEEEE..
//..SSSSSSSSSS..EEEEEEEEEE..RRRRRRRRRRR.VVVV....VVV..III..CCCCCCCCCCC..EEEEEEEEEE..
//..SSS...SSSS..EEE.........RRR.....RRR.VVVV...VVVV..III..CCCC...CCCC..EEE.........
//..SSSS........EEE.........RRR.....RRR..VVV...VVVV..III.CCCC.....CC...EEE.........
//..SSSSSSS.....EEEEEEEEEE..RRRRRRRRRRR..VVVV..VVV...III.CCCC..........EEEEEEEEEE..
//...SSSSSSSS...EEEEEEEEEE..RRRRRRRRRR...VVVV.VVVV...III.CCCC..........EEEEEEEEEE..
//.....SSSSSSS..EEEEEEEEEE..RRRRRRRR......VVV.VVVV...III.CCCC..........EEEEEEEEEE..
//.........SSSS.EEE.........RRR..RRRR.....VVVVVVV....III.CCCC.....CC...EEE.........
//.SSSS....SSSS.EEE.........RRR...RRRR....VVVVVVV....III..CCCC...CCCC..EEE.........
//..SSSSSSSSSS..EEEEEEEEEEE.RRR....RRRR....VVVVV.....III..CCCCCCCCCCC..EEEEEEEEEE..
//..SSSSSSSSSS..EEEEEEEEEEE.RRR....RRRR....VVVVV.....III...CCCCCCCCC...EEEEEEEEEE..
//....SSSSSS....EEEEEEEEEEE.RRR.....RRRR...VVVVV.....III.....CCCCCC....EEEEEEEEEE..
//.................................................................................

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
            img class="h-20 w-20" alt="check" src="/assets/images/icon/check.svg";
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

//.....................................................
//.TTTTTTTTTTT.EEEEEEEEEE...SSSSSSSS..STTTTTTTTTT.III..
//.TTTTTTTTTTT.EEEEEEEEEE..SSSSSSSSSS.STTTTTTTTTT.III..
//.TTTTTTTTTTT.EEEEEEEEEE..SSSS..SSSS.STTTTTTTTTT.III..
//.....TTT.....EEE.........SSS....SSS.....TTT.....III..
//.....TTT.....EEE.........SSSSS..........TTT.....III..
//.....TTT.....EEEEEEEEEE..SSSSSSSSS......TTT.....III..
//.....TTT.....EEEEEEEEEE...SSSSSSSSS.....TTT.....III..
//.....TTT.....EEEEEEEEEE.....SSSSSSSS....TTT.....III..
//.....TTT.....EEE................SSSS....TTT.....III..
//.....TTT.....EEE........ESSS....SSSS....TTT.....III..
//.....TTT.....EEEEEEEEEEEESSSSSSSSSSS....TTT.....III..
//.....TTT.....EEEEEEEEEEE.SSSSSSSSSS.....TTT.....III..
//.....TTT.....EEEEEEEEEEE..SSSSSSSS......TTT.....III..
//.....................................................

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
              img class="h-6 w-6" alt="star" src="/assets/images/icon/star.svg";
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
