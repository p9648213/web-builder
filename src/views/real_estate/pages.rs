use maud::{html, Markup, PreEscaped, DOCTYPE};

use crate::views::real_estate::{
    components, head::render_main_head, search_components, shared_components,
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
                (shared_components::render_nav_bar())
                main {
                    (components::render_home_banner())
                    (components::render_home_search_box())
                    (components::render_hot_property())
                    (components::render_our_services())
                    (components::render_testimonial())
                    (components::render_contact())
                }
                (shared_components::render_footer())
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
                (shared_components::render_nav_bar())
                main {
                    (search_components::render_search_box())
                    (search_components::render_search_result())
                }
                (shared_components::render_footer())
                div id="toast" {}
            }
        }
    }
}
