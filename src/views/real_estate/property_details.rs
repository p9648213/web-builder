use maud::{html, Markup, PreEscaped};

use crate::{
    controllers::real_estate::pages::PropertyQuery,
    models::rso_data::{Property, PropertyPicture, TextOrNum},
    views::icons::{
        basura_tax_gray_icon, bathroom_gray_icon, bedroom_gray_icon, buit_size_gray_icon,
        community_fee_gray_icon, feature_category_icon, feature_climate_control_icon,
        feature_condition_icon, feature_features_icon, feature_furniture_icon, feature_garden_icon,
        feature_kitchen_icon, feature_orientation_icon, feature_parking_icon, feature_pool_icon,
        feature_security_icon, feature_setting_icon, feature_utilities_icon, feature_view_icon,
        ibi_tax_gray_icon, plot_size_gray_icon, terrace_size_gray_icon, useful_size_gray_icon,
    },
};

pub fn render_pictures_slider(pictures: &Vec<PropertyPicture>) -> Markup {
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

pub fn render_property_details(property_query: PropertyQuery) -> Markup {
    if property_query.id.is_none() || property_query.listing_type.is_none() {
        return html! {
          div class="mt-25 py-15 text-center" {
            div {
              "Property not found"
            }
          }
        };
    }

    let property_id = property_query.id.unwrap();
    let listing_type = property_query.listing_type.unwrap();

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
          hx-get=(format!("/rso/property?id={}&type={}", property_id, listing_type))
          hx-target="#property-detail"
          hx-trigger="load"
          class="flex flex-col justify-center items-center gap-10 w-full max-w-360"
        {
          "Loading..."
        }
      }
    }
}

pub fn render_detail(property: &Property) -> Markup {
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

    html! {
      (render_pictures_slider(&property.pictures.picture))
      div class="flex flex-col gap-12 w-fit max-w-285" {
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
                  (render_grid_item("Built Size", built_size.as_str(), Some("m²") , Some(buit_size_gray_icon())))
                }
                @if plot_size != "0" {
                  (render_grid_item("Plot Size", plot_size.as_str(), Some("m²") , Some(plot_size_gray_icon())))
                }
                @if usefull_size != "0" {
                  (render_grid_item("Useful Size", usefull_size.as_str(), Some("m²") , Some(useful_size_gray_icon())))
                }
                @if terrace_size != "0" {
                  (render_grid_item("Terrace Size", terrace_size.as_str(), Some("m²") , Some(terrace_size_gray_icon())))
                }
                @if property.bedrooms != "0" {
                  (render_grid_item("Bedrooms", property.bedrooms.as_str(), None , Some(bedroom_gray_icon())))
                }
                @if property.bathrooms != "0" {
                  (render_grid_item("Bathrooms", property.bathrooms.as_str(), None , Some(bathroom_gray_icon())))
                }
              }
            }
            @if property.ibi_fee_year != "0" || property.basura_tax_year != "0" || property.community_fee_year != "0" {
              div class="flex flex-col gap-4" {
                div class="font-bold text-lg" { "Taxes" }
                div class="gap-6 grid grid-cols-4" {
                  @if property.ibi_fee_year != "0" {
                    (render_grid_item("IBI", property.ibi_fee_year.as_str(), Some("€/year") , Some(ibi_tax_gray_icon())))
                  }
                  @if property.basura_tax_year != "0" {
                    (render_grid_item("Basura Tax", property.basura_tax_year.as_str(), Some("€/year") , Some(basura_tax_gray_icon())))
                  }
                  @if property.community_fee_year != "0" {
                    (render_grid_item("Community Fee", property.community_fee_year.as_str(), Some("€/year") , Some(community_fee_gray_icon())))
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
                  (render_grid_item("Consumption", property.energy_rating.energy_value.as_str(), Some(" kg CO₂/m² per year"), None))
                  (render_grid_item("Emissions", property.energy_rating.co2_value.as_str(), Some(" kWh/m² per year"), None))
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
                      "Setting" => (feature_setting_icon()),
                      "Orientation" => (feature_orientation_icon()),
                      "Condition" => (feature_condition_icon()),
                      "Pool" => (feature_pool_icon()),
                      "Climate Control" => (feature_climate_control_icon()),
                      "Views" => (feature_view_icon()),
                      "Features" => (feature_features_icon()),
                      "Furniture" => (feature_furniture_icon()),
                      "Kitchen" => (feature_kitchen_icon()),
                      "Garden" => (feature_garden_icon()),
                      "Security" => (feature_security_icon()),
                      "Parking" => (feature_parking_icon()),
                      "Utilities" => (feature_utilities_icon()),
                      "Category" => (feature_category_icon()),
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

pub fn render_grid_item(
    label: &str,
    value: &str,
    dimension: Option<&str>,
    icon: Option<Markup>,
) -> Markup {
    html! {
      div class="flex flex-col gap-2" {
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
