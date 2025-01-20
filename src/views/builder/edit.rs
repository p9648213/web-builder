use maud::{html, Markup, DOCTYPE};

use crate::{
    controllers::builder::pages::{BuilderSection, BuilderStyle},
    views::{
        builder::head::render_main_builder_header,
        icons::{footer_back_icon, footer_next_icon},
    },
};

pub struct LayoutItem<'a> {
    pub layout_no: i32,
    pub layout_img_url: &'a str,
    pub preview_img_url: &'a str,
}

pub struct SUBNAV<'a> {
    title: &'a str,
    path: &'a str,
}

const MAIN_NAV_ITEM: [&str; 4] = ["CHOOSE STYLE", "BRANDING", "CONTENT", "ADVANCED SETTINGS"];

const CHOOSE_STYLE_NAV: [SUBNAV; 6] = [
    SUBNAV {
        title: "Header",
        path: "header",
    },
    SUBNAV {
        title: "Footer",
        path: "footer",
    },
    SUBNAV {
        title: "Home",
        path: "home",
    },
    SUBNAV {
        title: "Search Result",
        path: "search-result",
    },
    SUBNAV {
        title: "Property Details",
        path: "property-details",
    },
    SUBNAV {
        title: "Contact",
        path: "contact",
    },
];

pub const HEADER_LAYOUT: [LayoutItem; 4] = [
    LayoutItem {
        layout_no: 1,
        layout_img_url: "/assets/images/builder/header-layout-1.webp",
        preview_img_url: "/assets/images/builder/header-preview-1.webp",
    },
    LayoutItem {
        layout_no: 2,
        layout_img_url: "/assets/images/builder/header-layout-2.webp",
        preview_img_url: "/assets/images/builder/header-preview-2.webp",
    },
    LayoutItem {
        layout_no: 3,
        layout_img_url: "/assets/images/builder/header-layout-3.webp",
        preview_img_url: "/assets/images/builder/header-preview-3.webp",
    },
    LayoutItem {
        layout_no: 4,
        layout_img_url: "/assets/images/builder/header-layout-4.webp",
        preview_img_url: "/assets/images/builder/header-preview-4.webp",
    },
];

pub const FOOTER_LAYOUT: [LayoutItem; 4] = [
    LayoutItem {
        layout_no: 1,
        layout_img_url: "/assets/images/builder/footer-layout-1.webp",
        preview_img_url: "/assets/images/builder/footer-preview-1.webp",
    },
    LayoutItem {
        layout_no: 2,
        layout_img_url: "/assets/images/builder/footer-layout-2.webp",
        preview_img_url: "/assets/images/builder/footer-preview-2.webp",
    },
    LayoutItem {
        layout_no: 3,
        layout_img_url: "/assets/images/builder/footer-layout-3.webp",
        preview_img_url: "/assets/images/builder/footer-preview-3.webp",
    },
    LayoutItem {
        layout_no: 4,
        layout_img_url: "/assets/images/builder/footer-layout-4.webp",
        preview_img_url: "/assets/images/builder/footer-preview-4.webp",
    },
];

pub const HOME_LAYOUT: [LayoutItem; 4] = [
    LayoutItem {
        layout_no: 1,
        layout_img_url: "/assets/images/builder/home-layout-1.webp",
        preview_img_url: "/assets/images/builder/home-preview-1.webp",
    },
    LayoutItem {
        layout_no: 2,
        layout_img_url: "/assets/images/builder/home-layout-2.webp",
        preview_img_url: "/assets/images/builder/home-preview-2.webp",
    },
    LayoutItem {
        layout_no: 3,
        layout_img_url: "/assets/images/builder/home-layout-3.webp",
        preview_img_url: "/assets/images/builder/home-preview-3.webp",
    },
    LayoutItem {
        layout_no: 4,
        layout_img_url: "/assets/images/builder/home-layout-4.webp",
        preview_img_url: "/assets/images/builder/home-preview-4.webp",
    },
];

pub const SEARCH_LAYOUT: [LayoutItem; 4] = [
    LayoutItem {
        layout_no: 1,
        layout_img_url: "/assets/images/builder/search-layout-1.webp",
        preview_img_url: "/assets/images/builder/search-preview-1.webp",
    },
    LayoutItem {
        layout_no: 2,
        layout_img_url: "/assets/images/builder/search-layout-2.webp",
        preview_img_url: "/assets/images/builder/search-preview-2.webp",
    },
    LayoutItem {
        layout_no: 3,
        layout_img_url: "/assets/images/builder/search-layout-3.webp",
        preview_img_url: "/assets/images/builder/search-preview-3.webp",
    },
    LayoutItem {
        layout_no: 4,
        layout_img_url: "/assets/images/builder/search-layout-4.webp",
        preview_img_url: "/assets/images/builder/search-preview-4.webp",
    },
];

pub const PROPERTY_LAYOUT: [LayoutItem; 4] = [
    LayoutItem {
        layout_no: 1,
        layout_img_url: "/assets/images/builder/property-layout-1.webp",
        preview_img_url: "/assets/images/builder/property-preview-1.webp",
    },
    LayoutItem {
        layout_no: 2,
        layout_img_url: "/assets/images/builder/property-layout-2.webp",
        preview_img_url: "/assets/images/builder/property-preview-2.webp",
    },
    LayoutItem {
        layout_no: 3,
        layout_img_url: "/assets/images/builder/property-layout-3.webp",
        preview_img_url: "/assets/images/builder/property-preview-3.webp",
    },
    LayoutItem {
        layout_no: 4,
        layout_img_url: "/assets/images/builder/property-layout-4.webp",
        preview_img_url: "/assets/images/builder/property-preview-4.webp",
    },
];

pub const CONTACT_LAYOUT: [LayoutItem; 4] = [
    LayoutItem {
        layout_no: 1,
        layout_img_url: "/assets/images/builder/contact-layout-1.webp",
        preview_img_url: "/assets/images/builder/contact-preview-1.webp",
    },
    LayoutItem {
        layout_no: 2,
        layout_img_url: "/assets/images/builder/contact-layout-2.webp",
        preview_img_url: "/assets/images/builder/contact-preview-2.webp",
    },
    LayoutItem {
        layout_no: 3,
        layout_img_url: "/assets/images/builder/contact-layout-3.webp",
        preview_img_url: "/assets/images/builder/contact-preview-3.webp",
    },
    LayoutItem {
        layout_no: 4,
        layout_img_url: "/assets/images/builder/contact-layout-4.webp",
        preview_img_url: "/assets/images/builder/contact-preview-4.webp",
    },
];

pub fn render_edit_style_page(
    section: BuilderSection,
    style: BuilderStyle,
    theme: i32,
    setting_id: i32,
    user_id: i32,
    website_id: i32,
    authenticity_token: String,
) -> Markup {
    let highlight_nav_index = match section {
        BuilderSection::ChooseStyle => 1,
        BuilderSection::Branding => 2,
        BuilderSection::Content => 3,
        BuilderSection::AdvancedSetting => 4,
    };

    let (highlight_sub_nav_index, document_title, right_content_title, style_items, style_name) =
        match style {
            BuilderStyle::Header => (
                1,
                "Edit Header",
                "Choose your Header layout",
                HEADER_LAYOUT,
                "header",
            ),
            BuilderStyle::Footer => (
                2,
                "Edit Footer",
                "Choose your Footer layout",
                FOOTER_LAYOUT,
                "footer",
            ),
            BuilderStyle::Home => (
                3,
                "Edit Home",
                "Choose your Home page layout",
                HOME_LAYOUT,
                "home",
            ),
            BuilderStyle::SearchResult => (
                4,
                "Edit Search Result",
                "Choose your Search result layout",
                SEARCH_LAYOUT,
                "search-result",
            ),
            BuilderStyle::PropertyDetail => (
                5,
                "Edit Property Detail",
                "Choose your Property Detail layout",
                PROPERTY_LAYOUT,
                "property-details",
            ),
            BuilderStyle::Contact => (
                6,
                "Edit Contact",
                "Choose your Contact us layout",
                CONTACT_LAYOUT,
                "contact",
            ),
        };

    html! {
        (DOCTYPE)
        html class="h-full" {
          head {
            (render_main_builder_header())
          }
          body class="grid h-full" style="grid-template: auto 1fr auto / auto 1fr auto;" hx-boost="true" {
              title {
                  (document_title)
              }
              (render_nav_bar(highlight_nav_index))
              main class="grid grid-cols-10 col-[1/4] grow" {
                div class="col-span-7 bg-[#F3F3F3]" {
                  (render_left_content(highlight_sub_nav_index, &style_items, theme, website_id))
                }
                div class="col-span-3" {
                  (render_right_content(right_content_title, &style_items, style_name, theme, setting_id, user_id, authenticity_token))
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
            img class="h-full" src="/assets/images/builder/builder-logo.svg" alt="logo" ;
          }
          div class="flex justify-center items-center w-full" {
            ul class="flex gap-36 font-bold" {
              @for (index, item) in MAIN_NAV_ITEM.into_iter().enumerate() {
                @if index + 1 == highlight_index {
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
              "Next" (footer_next_icon())
            }
          }
        }
      }
    }
}

pub fn render_left_content(
    hightlight_sub_nav_index: usize,
    style_items: &[LayoutItem; 4],
    theme: i32,
    website_id: i32,
) -> Markup {
    html! {
      (render_sub_nav(hightlight_sub_nav_index, website_id))
      (render_preview_image(&style_items, theme, None))
    }
}

pub fn render_right_content(
    title: &str,
    style_items: &[LayoutItem; 4],
    style_name: &str,
    theme: i32,
    setting_id: i32,
    user_id: i32,
    authenticity_token: String,
) -> Markup {
    html! {
      div class="flex items-center p-8 h-full" {
        div class="flex flex-col gap-10" {
          span class="text-3xl" { (title) }
          (render_style_selection(&style_items, style_name, theme, setting_id, user_id, authenticity_token))
        }
      }
    }
}

pub fn render_sub_nav(highlight_index: usize, website_id: i32) -> Markup {
    html! {
      div class="flex justify-center p-8" {
        div class="flex gap-15 border-b border-b-gray-300 h-9 text-lg" {
          @for (index ,item) in CHOOSE_STYLE_NAV.into_iter().enumerate() {
            @if index + 1 == highlight_index {
              a class="border-b-2 border-b-blue-500 text-blue-500 cursor-pointer" { (item.title) }
            } @else {
              a
                href=(format!("/builder/edit/{}/style/{}", website_id, item.path))
                class="border-b-2 border-b-transparent hover:border-b-blue-500 hover:text-blue-500 transition-all duration-300 cursor-pointer"
              { (item.title) }
            }
          }
        }
      }
    }
}

pub fn render_style_selection(
    style_items: &[LayoutItem; 4],
    style_name: &str,
    choosen_style: i32,
    setting_id: i32,
    user_id: i32,
    authenticity_token: String,
) -> Markup {
    html! {
      div id="style-selection" class="gap-5 grid grid-cols-2" {
        @for item in style_items {
          @if choosen_style == item.layout_no {
            div class="border-3 border-blue-500 h-auto cursor-pointer" {
              img src=(item.layout_img_url) alt=(format!("{}-layout-{}", style_name, item.layout_no));
            }
          } @else {
            form
              hx-patch=(format!("/builder/edit/{}/{}/{}/{}", user_id, setting_id, style_name, item.layout_no))
              hx-trigger="click"
              hx-target="#style-selection"
              hx-swap="outerHTML"
            {
              div class="border-3 border-transparent hover:border-blue-500 cursor-pointer" {
                img src=(item.layout_img_url) alt=(format!("{}-layout-{}", style_name, item.layout_no));
              }
              input type="hidden" name="authenticity_token" value=(authenticity_token);
            }
          }
        }
      }
    }
}

pub fn render_preview_image(
    style_items: &[LayoutItem; 4],
    choosen_style: i32,
    oob_swap: Option<&str>,
) -> Markup {
    html! {
      div hx-swap-oob=[oob_swap] id="preview-image" class="flex justify-center" {
        @for item in style_items {
          @if choosen_style == item.layout_no {
            img src=(item.preview_img_url) alt=(format!("header-preview-{}", item.layout_no));
          }
        }
      }
    }
}
