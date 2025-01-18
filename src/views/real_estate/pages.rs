use maud::{html, Markup, PreEscaped, DOCTYPE};

use crate::{
    controllers::real_estate::pages::{PropertyQuery, SearchQuery},
    views::real_estate::{head::render_main_head, home, property_details, search_result, shared},
};

pub fn render_home_page(header_theme: i32, footer_theme: i32) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            (render_main_head())
            body hx-boost="true" class="scroll-smooth" {
                title {
                    "Home"
                }
                (PreEscaped(r#"
                    <script>0</script>
                "#))
                @match header_theme {
                    1 => (shared::render_nav_bar_1()),
                    2 => (shared::render_nav_bar_2()),
                    3 => (shared::render_nav_bar_3()),
                    4 => (shared::render_nav_bar_4()),
                    _ => (shared::render_nav_bar_1())
                }
                main {
                    (home::render_home_banner())
                    (home::render_home_search_box())
                    (home::render_hot_properties())
                    (home::render_our_services())
                    (home::render_testimonial())
                    (shared::render_contact())
                }
                @match footer_theme {
                    1 => (shared::render_footer_1()),
                    2 => (shared::render_footer_2()),
                    3 => (shared::render_footer_3()),
                    4 => (shared::render_footer_4()),
                    _ => (shared::render_footer_1())
                }
                div id="toast" {}
            }
        }
    }
}

pub fn render_search_result_page(
    search_query: SearchQuery,
    header_theme: i32,
    footer_theme: i32,
    search_theme: i32,
) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            (render_main_head())
            body hx-boost="true" class="scroll-smooth" {
                title {
                    "Search Result"
                }
                (PreEscaped(r#"
                    <script>0</script>
                "#))
                @match header_theme {
                    1 => (shared::render_nav_bar_1()),
                    2 => (shared::render_nav_bar_2()),
                    3 => (shared::render_nav_bar_3()),
                    4 => (shared::render_nav_bar_4()),
                    _ => (shared::render_nav_bar_1())
                }
                main {
                    div id="search-section" class="flex flex-col items-center min-h-screen invisible" {
                        @match search_theme {
                            1 => {
                                (search_result::render_search_box_1())
                                (search_result::render_search_result_1(search_query.page))
                            }
                            2 => {
                                div class="relative flex justify-between gap-10 mt-15 px-5 pb-30 w-full max-w-360" {
                                    (search_result::render_search_box_2())
                                    (search_result::render_search_result_2(search_query.page))
                                }
                            }
                            3 => {
                                (search_result::render_search_box_3())
                                (search_result::render_search_result_3(search_query.page))
                            }
                            4 => {

                                (search_result::render_search_box_4())
                                (search_result::render_search_result_4(search_query.page))
                            }
                            _ => {
                                (search_result::render_search_box_1())
                                (search_result::render_search_result_1(search_query.page))
                            }
                        }
                    }
                    (shared::render_contact())
                }
                @match footer_theme {
                    1 => (shared::render_footer_1()),
                    2 => (shared::render_footer_2()),
                    3 => (shared::render_footer_3()),
                    4 => (shared::render_footer_4()),
                    _ => (shared::render_footer_1())
                }
                div id="toast" {}
            }
        }
    }
}

pub fn render_property_details_page(
    property_query: PropertyQuery,
    header_theme: i32,
    footer_theme: i32,
) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            (render_main_head())
            body hx-boost="true" class="scroll-smooth" {
                title {
                    "Property Details"
                }
                (PreEscaped(r#"
                    <script>0</script>
                "#))
                @match header_theme {
                    1 => (shared::render_nav_bar_1()),
                    2 => (shared::render_nav_bar_2()),
                    3 => (shared::render_nav_bar_3()),
                    4 => (shared::render_nav_bar_4()),
                    _ => (shared::render_nav_bar_1())
                }
                main {
                    (property_details::render_property_details(property_query))
                    (shared::render_contact())
                }
                @match footer_theme {
                    1 => (shared::render_footer_1()),
                    2 => (shared::render_footer_2()),
                    3 => (shared::render_footer_3()),
                    4 => (shared::render_footer_4()),
                    _ => (shared::render_footer_1())
                }
                div id="toast" {}
            }
        }
    }
}
