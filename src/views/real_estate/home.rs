use maud::PreEscaped;

use crate::views::real_estate::{
    components::{render_home_banner, render_home_search_box, render_hot_property, render_nav_bar, render_our_services},
    head::render_main_head,
};

pub fn render_home_page() -> maud::Markup {
    maud::html! {
        (maud::DOCTYPE)
        (render_main_head())
        body hx-boost="true" class="h-2000 scroll-smooth" {
            title {
                "Home"
            }
            (PreEscaped(r#"
                <script>0</script>
            "#))
            main {
                (render_nav_bar())
                (render_home_banner())
                (render_home_search_box())
                (render_hot_property())
                (render_our_services())
            }
            div id="toast" {}
        }
    }
}
