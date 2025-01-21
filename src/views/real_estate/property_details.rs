use maud::{html, Markup, PreEscaped};

use crate::{
    controllers::real_estate::pages::PropertyQuery,
    models::rso_data::{Property, PropertyPicture, TextOrNum},
};

//.............................................................
//.PPPPPPPPP...RRRRRRRRR.......OOOOOO.....PPPPPPPPP......1111..
//.PPPPPPPPPP..RRRRRRRRRRR...OOOOOOOOOO...PPPPPPPPPP.....1111..
//.PPPPPPPPPP..RRRRRRRRRRR..OOOOOOOOOOOO..PPPPPPPPPP...111111..
//.PPP....PPPP.RRR.....RRR..OOOO....OOOO..PPP....PPPP.1111111..
//.PPP....PPPP.RRR.....RRR..OOO......OOO..PPP....PPPP.1111111..
//.PPPPPPPPPP..RRRRRRRRRRR.ROOO......OOOO.PPPPPPPPPP..11.1111..
//.PPPPPPPPPP..RRRRRRRRRR..ROOO......OOOO.PPPPPPPPPP.....1111..
//.PPPPPPPPP...RRRRRRRR....ROOO......OOOO.PPPPPPPPP......1111..
//.PPP.........RRR..RRRR....OOO......OOO..PPP............1111..
//.PPP.........RRR...RRRR...OOOO....OOOO..PPP............1111..
//.PPP.........RRR....RRRR..OOOOOOOOOOOO..PPP............1111..
//.PPP.........RRR....RRRR...OOOOOOOOOO...PPP............1111..
//.PPP.........RRR.....RRRR....OOOOOO.....PPP............1111..
//.............................................................

pub fn render_property_details_1(property_query: &PropertyQuery) -> Markup {
    if property_query.id.is_none() || property_query.listing_type.is_none() {
        return html! {
          div class="mt-25 py-15 text-center" {
            div {
              "Property not found"
            }
          }
        };
    }

    let property_id = property_query.id.as_ref().unwrap();
    let listing_type = property_query.listing_type.as_ref().unwrap();

    html! {
      (PreEscaped(r#"
        <script type="module">
            import {scrollToTop} from "/assets/js/main.js";
            scrollToTop();
        </script>
      "#))
      div class="flex justify-center items-center mt-25 py-15" {
        div
          id="property-detail"
          hx-get=(format!("/rso/property?id={}&type={}&theme=1", property_id, listing_type))
          hx-target="#property-detail"
          hx-trigger="load"
          class="flex flex-col justify-center items-center gap-10 w-full max-w-360"
        {
          "Loading..."
        }
      }
    }
}

pub fn render_detail_1(property: &Property) -> Markup {
    let name = if property.newdev_name == "" {
        &property.property_type.name_type
    } else {
        &property.newdev_name
    };

    let price = match &property.price {
        TextOrNum::Text(price) => price,
        TextOrNum::Num(price) => &price.to_string(),
    };

    let built_size = match &property.built {
        TextOrNum::Text(built) => built.to_string(),
        TextOrNum::Num(built) => built.to_string(),
    };

    let plot_size = match &property.garden_plot {
        TextOrNum::Text(plot) => plot.to_string(),
        TextOrNum::Num(plot) => plot.to_string(),
    };

    let usefull_size = match &property.int_floor_space {
        TextOrNum::Text(usefull) => usefull.to_string(),
        TextOrNum::Num(usefull) => usefull.to_string(),
    };

    let terrace_size = match &property.terrace {
        TextOrNum::Text(terrace) => terrace.to_string(),
        TextOrNum::Num(terrace) => terrace.to_string(),
    };

    let built_size_gray_icon = html! {
        img class="w-5 h-5" alt="built size" src="/assets/images/icon/built-size-gray.svg";
    };

    let plot_size_gray_icon = html! {
        img class="w-5 h-5" alt="plot size" src="/assets/images/icon/plot-size-gray.svg";
    };

    let usefull_size_gray_icon = html! {
        img class="w-5 h-5" alt="usefull size" src="/assets/images/icon/usefull-size-gray.svg";
    };

    let terrace_size_gray_icon = html! {
        img class="w-5 h-5" alt="terrace size" src="/assets/images/icon/terrace-size-gray.svg";
    };

    let bedroom_gray_icon = html! {
        img class="w-5 h-5" alt="bedroom" src="/assets/images/icon/bed-gray.svg";
    };

    let bathroom_gray_icon = html! {
        img class="w-5 h-5" alt="bathroom" src="/assets/images/icon/bath-gray.svg";
    };

    let ibi_tax_gray_icon = html! {
        img class="w-5 h-5" alt="ibi_tax" src="/assets/images/icon/ibi-gray.svg";
    };

    let basura_tax_gray_icon = html! {
        img class="w-5 h-5" alt="basura_tax" src="/assets/images/icon/basura-gray.svg";
    };

    let community_fee_gray_icon = html! {
        img class="w-5 h-5" alt="community_fee" src="/assets/images/icon/community-gray.svg";
    };

    html! {
      (render_pictures_slider_1(&property.pictures.picture))
      div class="flex flex-col gap-12 w-fit max-w-290" {
        div class="flex justify-between font-bold text-xl" {
          div { (name.to_uppercase()) }
          div { (property.location) }
          div { (property.reference) }
        }
        div class="flex justify-between gap-15 border-slate-200 shadow-xs px-10 py-8 border rounded-lg" {
          div class="flex flex-col gap-4 max-w-130" {
            div class="font-bold text-lg" { "Description" }
            div class="text-[#868d9b] text-justify whitespace-pre-line" {
              (PreEscaped(html_escape::decode_html_entities(&property.description).replace("[IW]", "")))
            }
          }
          div class="flex flex-col gap-8" {
            div class="flex justify-between items-center" {
              div class="font-bold text-2xl text-blue-500" { (price) " €" }
              button class="bg-blue-500 hover:bg-blue-400 px-4 py-3 rounded-md font-bold text-white cursor-pointer" {
                "GET IN TOUCH"
              }
            }
            div class="flex flex-col gap-4" {
              div class="font-bold text-lg" { "Basic characteristics" }
              div class="gap-6 grid grid-cols-4" {
                @if built_size != "0" {
                  (render_grid_item_1("Built Size", built_size.as_str(), Some("m²") , Some(built_size_gray_icon)))
                }
                @if plot_size != "0" {
                  (render_grid_item_1("Plot Size", plot_size.as_str(), Some("m²") , Some(plot_size_gray_icon)))
                }
                @if usefull_size != "0" {
                  (render_grid_item_1("Useful Size", usefull_size.as_str(), Some("m²") , Some(usefull_size_gray_icon)))
                }
                @if terrace_size != "0" {
                  (render_grid_item_1("Terrace Size", terrace_size.as_str(), Some("m²") , Some(terrace_size_gray_icon)))
                }
                @if property.bedrooms != "0" {
                  (render_grid_item_1("Bedrooms", property.bedrooms.as_str(), None , Some(bedroom_gray_icon)))
                }
                @if property.bathrooms != "0" {
                  (render_grid_item_1("Bathrooms", property.bathrooms.as_str(), None , Some(bathroom_gray_icon)))
                }
              }
            }
            @if property.ibi_fee_year != "0" || property.basura_tax_year != "0" || property.community_fee_year != "0" {
              div class="flex flex-col gap-4" {
                div class="font-bold text-lg" { "Taxes" }
                div class="items-start gap-6 grid grid-cols-4" {
                  @if property.ibi_fee_year != "0" {
                    (render_grid_item_1("IBI", property.ibi_fee_year.as_str(), Some("€/year") , Some(ibi_tax_gray_icon)))
                  }
                  @if property.basura_tax_year != "0" {
                    (render_grid_item_1("Basura Tax", property.basura_tax_year.as_str(), Some("€/year") , Some(basura_tax_gray_icon)))
                  }
                  @if property.community_fee_year != "0" {
                    (render_grid_item_1("Community Fee", property.community_fee_year.as_str(), Some("€/year") , Some(community_fee_gray_icon)))
                  }
                }
              }
            }
            div class="flex flex-col gap-4" {
              div class="font-bold text-lg" { "Energy Certificate" }
              div class="gap-6 grid grid-cols-2" {
                @if property.energy_rating.co2_value == "" && property.energy_rating.energy_value == ""  {
                  div class="text-[#868d9b]" { "Under valuation" }
                } @else {
                  (render_grid_item_1("Consumption", property.energy_rating.energy_value.as_str(), Some(" kg CO₂/m² per year"), None))
                  (render_grid_item_1("Emissions", property.energy_rating.co2_value.as_str(), Some(" kWh/m² per year"), None))
                }
              }
            }
          }
        }
        div class="flex flex-col gap-10 border-slate-200 shadow-xs px-10 py-8 border rounded-lg" {
          div class="font-bold text-lg" { "Features" }
          div class="flex flex-col gap-8" {
            @for category in &property.property_features.category {
              div class="grid grid-cols-[11rem_1fr]" {
                div {
                  div class="flex items-center gap-3 font-bold" {
                    @match category.category_type.as_str() {
                      "Setting" => {
                        img class="w-6 h-6" src="/assets/images/icon/setting.svg" alt="setting";
                      },
                      "Orientation" => {
                        img class="w-6 h-6" src="/assets/images/icon/orientation.svg" alt="orientation";
                      },
                      "Condition" => {
                        img class="w-6 h-6" src="/assets/images/icon/condition.svg" alt="condition";
                      },
                      "Pool" => {
                        img class="w-6 h-6" src="/assets/images/icon/pool.svg" alt="pool";
                      },
                      "Climate Control" => {
                        img class="w-6 h-6" src="/assets/images/icon/climate.svg" alt="climate";
                      },
                      "Views" => {
                        img class="w-6 h-6" src="/assets/images/icon/view.svg" alt="view";
                      },
                      "Features" => {
                        img class="w-6 h-6" src="/assets/images/icon/feature.svg" alt="feature";
                      },
                      "Furniture" => {
                        img class="w-6 h-6" src="/assets/images/icon/furniture.svg" alt="furniture";
                      },
                      "Kitchen" => {
                        img class="w-6 h-6" src="/assets/images/icon/kitchen.svg" alt="kitchen";
                      },
                      "Garden" => {
                        img class="w-6 h-6" src="/assets/images/icon/garden.svg" alt="garden";
                      },
                      "Security" => {
                        img class="w-6 h-6" src="/assets/images/icon/security.svg" alt="security";
                      },
                      "Parking" => {
                        img class="w-6 h-6" src="/assets/images/icon/parking.svg" alt="parking";
                      },
                      "Utilities" => {
                        img class="w-6 h-6" src="/assets/images/icon/utilities.svg" alt="utilities";
                      },
                      "Category" => {
                        img class="w-6 h-6" src="/assets/images/icon/category.svg" alt="category";
                      },
                      _ => ""
                    }
                    div class="font-bold" {
                      (category.category_type) " :"
                    }
                  }
                }
                div class="flex flex-wrap gap-4 text-wrap" {
                  @for value in &category.category_value {
                    div {
                      "- " span { (value) }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
}

pub fn render_pictures_slider_1(pictures: &Vec<PropertyPicture>) -> Markup {
    html! {
      (PreEscaped(r#"
        <script type="module">
            import {setupPropertyPictureSlider} from "/assets/js/app/slider.js";
            setupPropertyPictureSlider();
        </script>
      "#))
      div class="flex justify-center bg-black rounded-lg w-full max-w-300 h-110 picture-container" {
        div class="relative w-200 h-full overflow-hidden picture-slider-container" {
          div class="flex w-full h-full transition-transform duration-500 picture-slider" {
            input type="hidden" value=(pictures.len());
            @for picture in pictures {
              img class="w-full h-full pointer-events-none object-contain shrink-0" src=(picture.picture_url);
            }
          }
          div class="bottom-2 left-[50%] absolute flex gap-2 -translate-x-[50%] overflow-hidden pictures-dots" {
            @for i in 0..pictures.len() as u8 {
              @if i == 0 {
                div class="bg-blue-500 p-1 rounded-full cursor-pointer" {}
              } @else {
                div class="bg-blue-200 p-1 rounded-full cursor-pointer" {}
              }
            }
          }
        }
      }
    }
}

pub fn render_grid_item_1(
    label: &str,
    value: &str,
    dimension: Option<&str>,
    icon: Option<Markup>,
) -> Markup {
    let grid_item_class = if label == "Community Fee" {
        "flex flex-col gap-2 col-span-2"
    } else {
        "flex flex-col gap-2"
    };

    html! {
      div class=(grid_item_class) {
        div class="flex items-end gap-2" {
          @if let Some(icon) = icon {
            (icon)
          }
          div class="text-[#868d9b] text-sm" { (label) }
        }
        div class="flex" {
          (value)
          @if let Some(dimension) = dimension {
            (dimension)
          }
        }
      }
    }
}

//...............................................................
//.PPPPPPPPP...RRRRRRRRR.......OOOOOO.....PPPPPPPPP....222222....
//.PPPPPPPPPP..RRRRRRRRRRR...OOOOOOOOOO...PPPPPPPPPP..22222222...
//.PPPPPPPPPP..RRRRRRRRRRR..OOOOOOOOOOOO..PPPPPPPPPP..222..222...
//.PPP....PPPP.RRR.....RRR..OOOO....OOOO..PPP....PPPPP222..2222..
//.PPP....PPPP.RRR.....RRR..OOO......OOO..PPP....PPPP......2222..
//.PPPPPPPPPP..RRRRRRRRRRR.ROOO......OOOO.PPPPPPPPPP.......222...
//.PPPPPPPPPP..RRRRRRRRRR..ROOO......OOOO.PPPPPPPPPP......2222...
//.PPPPPPPPP...RRRRRRRR....ROOO......OOOO.PPPPPPPPP......2222....
//.PPP.........RRR..RRRR....OOO......OOO..PPP...........2222.....
//.PPP.........RRR...RRRR...OOOO....OOOO..PPP..........2222......
//.PPP.........RRR....RRRR..OOOOOOOOOOOO..PPP.........2222.......
//.PPP.........RRR....RRRR...OOOOOOOOOO...PPP........P222222222..
//.PPP.........RRR.....RRRR....OOOOOO.....PPP........P222222222..
//...............................................................

pub fn render_property_details_2(property_query: &PropertyQuery) -> Markup {
    if property_query.id.is_none() || property_query.listing_type.is_none() {
        return html! {
          div class="mt-25 py-15 text-center" {
            div {
              "Property not found"
            }
          }
        };
    }

    let property_id = property_query.id.as_ref().unwrap();
    let listing_type = property_query.listing_type.as_ref().unwrap();

    html! {
      (PreEscaped(r#"
        <script type="module">
            import {scrollToTop} from "/assets/js/main.js";
            scrollToTop();
        </script>
      "#))
      div class="flex justify-center items-center mt-25 py-15" {
        div
          id="property-detail"
          hx-get=(format!("/rso/property?id={}&type={}&theme=2", property_id, listing_type))
          hx-target="#property-detail"
          hx-trigger="load"
          class="flex flex-col justify-center items-center gap-10 w-full max-w-360"
        {
          "Loading..."
        }
      }
    }
}

pub fn render_detail_2(property: &Property) -> Markup {
    let name = if property.newdev_name == "" {
        &property.property_type.name_type
    } else {
        &property.newdev_name
    };

    let price = match &property.price {
        TextOrNum::Text(price) => price,
        TextOrNum::Num(price) => &price.to_string(),
    };

    let built_size = match &property.built {
        TextOrNum::Text(built) => built.to_string(),
        TextOrNum::Num(built) => built.to_string(),
    };

    let plot_size = match &property.garden_plot {
        TextOrNum::Text(plot) => plot.to_string(),
        TextOrNum::Num(plot) => plot.to_string(),
    };

    let usefull_size = match &property.int_floor_space {
        TextOrNum::Text(usefull) => usefull.to_string(),
        TextOrNum::Num(usefull) => usefull.to_string(),
    };

    let terrace_size = match &property.terrace {
        TextOrNum::Text(terrace) => terrace.to_string(),
        TextOrNum::Num(terrace) => terrace.to_string(),
    };

    let mut characteristics_count = 0;

    if built_size != "0" {
        characteristics_count += 1;
    }

    if plot_size != "0" {
        characteristics_count += 1;
    }

    if usefull_size != "0" {
        characteristics_count += 1;
    }

    if terrace_size != "0" {
        characteristics_count += 1;
    }

    if property.bedrooms != "0" {
        characteristics_count += 1;
    }

    if property.bathrooms != "0" {
        characteristics_count += 1;
    }

    let mut taxes_count = 0;

    if property.ibi_fee_year != "0" {
        taxes_count += 1;
    }

    if property.basura_tax_year != "0" {
        taxes_count += 1;
    }

    if property.community_fee_year != "0" {
        taxes_count += 1;
    }

    let characteristics_grid_class = if characteristics_count >= 4 {
        "gap-3.5 grid grid-cols-2 grid-rows-4 grid-flow-col".to_string()
    } else {
        format!(
            "gap-3.5 grid grid-cols-2 grid-rows-{} grid-flow-col",
            characteristics_count
        )
    };

    let taxes_grid_class = if taxes_count >= 2 {
        "gap-3.5 grid grid-cols-2 grid-rows-2 grid-flow-col".to_string()
    } else {
        format!(
            "gap-3.5 grid grid-cols-2 grid-rows-{} grid-flow-col",
            taxes_count
        )
    };

    html! {
      div class="flex flex-col gap-15 w-full max-w-340" {
        div class="flex justify-between gap-10" {
          (render_pictures_slider_2(&property.pictures.picture))
          div class="flex flex-col flex-2 justify-center gap-6" {
            div class="flex justify-between gap-5 w-full" {
              span class="font-bold text-xl" { "Ref: " (property.reference) }
              button class="bg-blue-500 hover:bg-blue-400 px-4 py-3 rounded-md h-fit font-bold text-white cursor-pointer" {
                "GET IN TOUCH"
              }
            }
            div class="flex flex-col gap-2.5" {
              span class="font-bold text-lg" { (property.location) }
              span { (name) }
              span class="font-bold text-3xl text-blue-500" { (price) " €" }
            }
            div class="flex flex-col gap-3" {
              span class="font-bold text-lg" { "Basic characteristics" }
              div class=(characteristics_grid_class) {
                @if built_size != "0" {
                  div {
                    span class="text-[#868D9B]" {"Built Size: "}
                    (built_size) "m²"
                  }
                }
                @if plot_size != "0" {
                  div {
                    span class="text-[#868D9B]" {"Plot Size: "}
                    (plot_size) "m²"
                  }
                }
                @if usefull_size != "0" {
                  div {
                    span class="text-[#868D9B]" {"Useful Size: "}
                    (usefull_size) "m²"
                  }
                }
                @if terrace_size != "0" {
                  div {
                    span class="text-[#868D9B]" {"Terrace Size: "}
                    (terrace_size) "m²"
                  }
                }
                @if property.bedrooms != "0" {
                  div {
                    span class="text-[#868D9B]" {"Bedrooms: "}
                    (property.bedrooms)
                  }
                }
                @if property.bathrooms != "0" {
                  div {
                    span class="text-[#868D9B]" {"Bathrooms: "}
                    (property.bathrooms)
                  }
                }
              }
            }
            @if taxes_count > 0 {
              div class="flex flex-col gap-3" {
                span class="font-bold text-lg" { "Taxes" }
                div class=(taxes_grid_class) {
                  @if property.ibi_fee_year != "0"{
                    div {
                      span class="text-[#868D9B]" {"IBI: "}
                      (property.ibi_fee_year) "€/year"
                    }
                  }
                  @if property.basura_tax_year != "0" {
                    div {
                      span class="text-[#868D9B]" {"Basura Tax: "}
                      (property.basura_tax_year) "€/year"
                    }
                  }
                  @if property.community_fee_year != "0" {
                    div {
                      span class="text-[#868D9B]" {"Community Fee: "}
                      (property.community_fee_year) "€/year"
                    }
                  }
                }
              }
            }
            div class="flex flex-col gap-3" {
              span class="font-bold text-lg" { "Energy Certificate" }
              @if property.energy_rating.co2_value == "" && property.energy_rating.energy_value == ""  {
                div class="text-[#868d9b]" { "Under valuation" }
              } @else {
                div {
                  span class="text-[#868D9B]" {"Consumption: "}
                  (property.energy_rating.energy_value) " kg CO₂/m² per year"
                }
                div {
                  span class="text-[#868D9B]" {"Emissions: "}
                  (property.energy_rating.co2_value) " kg CO₂/m² per year"
                }
              }
            }
          }
        }
        div class="flex flex-col gap-5" {
          span class="font-bold text-lg" { "Description" }
          p class="text-[#868d9b] text-justify whitespace-pre-line" {
            (PreEscaped(html_escape::decode_html_entities(&property.description).replace("[IW]", "")))
          }
        }
        div class="flex flex-col gap-5" {
          span class="font-bold text-lg" { "Features" }
          div class="gap-x-15 gap-y-5 grid grid-cols-3" {
            @for category in &property.property_features.category {
              div class="flex gap-2" {
                img class="w-6 h-6" src="/assets/images/icon/check.svg" alt="check";
                span class="font-bold text-[#868d9b] whitespace-nowrap" { (category.category_type) ":" }
                div {
                  @for (index, value) in category.category_value.iter().enumerate() {
                    span class="inline-block whitespace-pre-line" { (value) }
                    @if index < category.category_value.len() - 1 {
                      span {", "}
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
}

pub fn render_pictures_slider_2(pictures: &Vec<PropertyPicture>) -> Markup {
    html! {
      (PreEscaped(r#"
      <script type="module">
          import {setupPropertyPictureSlider} from "/assets/js/app/slider.js";
          setupPropertyPictureSlider();
      </script>
    "#))
      div class="flex-3 justify-center items-center bg-black rounded-lg h-160 picture-container" {
        div class="relative rounded-lg h-full overflow-hidden picture-slider-container" {
          div class="flex w-full h-full transition-transform duration-500 picture-slider" {
            input type="hidden" value=(pictures.len());
            @for picture in pictures {
              img class="w-full h-full pointer-events-none object-contain shrink-0" src=(picture.picture_url);
            }
          }
        }
      }
    }
}

//...............................................................
//.PPPPPPPPP...RRRRRRRRR.......OOOOOO.....PPPPPPPPP....33333.....
//.PPPPPPPPPP..RRRRRRRRRRR...OOOOOOOOOO...PPPPPPPPPP..3333333....
//.PPPPPPPPPP..RRRRRRRRRRR..OOOOOOOOOOOO..PPPPPPPPPP..333.3333...
//.PPP....PPPP.RRR.....RRR..OOOO....OOOO..PPP....PPPP.333..333...
//.PPP....PPPP.RRR.....RRR..OOO......OOO..PPP....PPPP.....3333...
//.PPPPPPPPPP..RRRRRRRRRRR.ROOO......OOOO.PPPPPPPPPP.....3333....
//.PPPPPPPPPP..RRRRRRRRRR..ROOO......OOOO.PPPPPPPPPP.....3333....
//.PPPPPPPPP...RRRRRRRR....ROOO......OOOO.PPPPPPPPP........333...
//.PPP.........RRR..RRRR....OOO......OOO..PPP..............3333..
//.PPP.........RRR...RRRR...OOOO....OOOO..PPP........P333..3333..
//.PPP.........RRR....RRRR..OOOOOOOOOOOO..PPP.........333..333...
//.PPP.........RRR....RRRR...OOOOOOOOOO...PPP.........33333333...
//.PPP.........RRR.....RRRR....OOOOOO.....PPP..........33333.....
//...............................................................

pub fn render_property_details_3(property_query: &PropertyQuery) -> Markup {
    if property_query.id.is_none() || property_query.listing_type.is_none() {
        return html! {
          div class="mt-25 py-15 text-center" {
            div {
              "Property not found"
            }
          }
        };
    }

    let property_id = property_query.id.as_ref().unwrap();
    let listing_type = property_query.listing_type.as_ref().unwrap();

    html! {
      (PreEscaped(r#"
        <script type="module">
            import {scrollToTop} from "/assets/js/main.js";
            scrollToTop();
        </script>
      "#))
      div class="flex justify-center items-center mt-25 py-15" {
        div
          id="property-detail"
          hx-get=(format!("/rso/property?id={}&type={}&theme=3", property_id, listing_type))
          hx-target="#property-detail"
          hx-trigger="load"
          class="flex flex-col justify-center items-center gap-10 w-full max-w-360"
        {
          "Loading..."
        }
      }
    }
}

pub fn render_detail_3(property: &Property) -> Markup {
    html! {
      "Detail 3"
    }
}

//...............................................................
//.PPPPPPPPP...RRRRRRRRR.......OOOOOO.....PPPPPPPPP.......444....
//.PPPPPPPPPP..RRRRRRRRRRR...OOOOOOOOOO...PPPPPPPPPP.....4444....
//.PPPPPPPPPP..RRRRRRRRRRR..OOOOOOOOOOOO..PPPPPPPPPP.....4444....
//.PPP....PPPP.RRR.....RRR..OOOO....OOOO..PPP....PPPP...44444....
//.PPP....PPPP.RRR.....RRR..OOO......OOO..PPP....PPPP..444444....
//.PPPPPPPPPP..RRRRRRRRRRR.ROOO......OOOO.PPPPPPPPPP...444444....
//.PPPPPPPPPP..RRRRRRRRRR..ROOO......OOOO.PPPPPPPPPP..444.444....
//.PPPPPPPPP...RRRRRRRR....ROOO......OOOO.PPPPPPPPP..P44..444....
//.PPP.........RRR..RRRR....OOO......OOO..PPP........P444444444..
//.PPP.........RRR...RRRR...OOOO....OOOO..PPP........P444444444..
//.PPP.........RRR....RRRR..OOOOOOOOOOOO..PPP.............444....
//.PPP.........RRR....RRRR...OOOOOOOOOO...PPP.............444....
//.PPP.........RRR.....RRRR....OOOOOO.....PPP.............444....
//...............................................................

pub fn render_property_details_4(property_query: &PropertyQuery) -> Markup {
    if property_query.id.is_none() || property_query.listing_type.is_none() {
        return html! {
          div class="mt-25 py-15 text-center" {
            div {
              "Property not found"
            }
          }
        };
    }

    let property_id = property_query.id.as_ref().unwrap();
    let listing_type = property_query.listing_type.as_ref().unwrap();

    html! {
      (PreEscaped(r#"
        <script type="module">
            import {scrollToTop} from "/assets/js/main.js";
            scrollToTop();
        </script>
      "#))
      div class="flex justify-center items-center mt-25 py-15" {
        div
          id="property-detail"
          hx-get=(format!("/rso/property?id={}&type={}&theme=4", property_id, listing_type))
          hx-target="#property-detail"
          hx-trigger="load"
          class="flex flex-col justify-center items-center gap-10 w-full max-w-360"
        {
          "Loading..."
        }
      }
    }
}

pub fn render_detail_4(property: &Property) -> Markup {
    html! {
      "Detail 4"
    }
}
