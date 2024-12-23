use maud::{html, Markup, PreEscaped, DOCTYPE};

use crate::views::real_estate::{
    components::{
        render_contact, render_footer, render_home_banner, render_home_search_box,
        render_hot_property, render_nav_bar, render_our_services, render_testimonial,
    },
    head::render_main_head,
    search_components,
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
                (render_nav_bar())
                main {
                    (render_home_banner())
                    (render_home_search_box())
                    (render_hot_property())
                    (render_our_services())
                    (render_testimonial())
                    (render_contact())
                }
                (render_footer())
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
                (render_nav_bar())
                main {
                    (search_components::render_search_box())
                }
                (render_footer())
                div id="toast" {}
            }
        }
    }
}
