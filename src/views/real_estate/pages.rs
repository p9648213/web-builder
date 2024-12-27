use maud::{html, Markup, PreEscaped, DOCTYPE};

use crate::views::real_estate::{head::render_main_head, home, search_result, shared};

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
                (shared::render_nav_bar())
                main {
                    (home::render_home_banner())
                    (home::render_home_search_box())
                    (home::render_hot_property())
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

pub fn render_search_result_page() -> Markup {
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
                (shared::render_nav_bar())
                main {
                    (search_result::render_search_box())
                    (search_result::render_search_result())
                    (shared::render_contact())
                }
                (shared::render_footer())
                div id="toast" {}
            }
        }
    }
}
