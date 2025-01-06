use maud::{html, Markup, PreEscaped, DOCTYPE};

use crate::{
    controllers::real_estate::pages::{PropertyQuery, SearchQuery},
    views::real_estate::{head::render_main_head, home, property_details, search_result, shared},
};

pub fn render_home_page() -> Markup {
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
                (shared::render_nav_bar_1())
                main {
                    (home::render_home_banner())
                    (home::render_home_search_box())
                    (home::render_hot_properties())
                    (home::render_our_services())
                    (home::render_testimonial())
                    (shared::render_contact())
                }
                (shared::render_footer())
                div id="toast" {}
            }
        }
    }
}

pub fn render_search_result_page(search_query: SearchQuery) -> Markup {
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
                (shared::render_nav_bar_1())
                main {
                    (search_result::render_search_box())
                    (search_result::render_search_result(search_query.page))
                    (shared::render_contact())
                }
                (shared::render_footer())
                div id="toast" {}
            }
        }
    }
}

pub fn render_property_details_page(property_query: PropertyQuery) -> Markup {
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
                (shared::render_nav_bar_1())
                main {
                    (property_details::render_property_details(property_query))
                    (shared::render_contact())
                }
                (shared::render_footer())
                div id="toast" {}
            }
        }
    }
}
