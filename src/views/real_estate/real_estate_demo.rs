use crate::views::real_estate::header::render_main_app_header;

pub fn render_real_estate_demo_page() -> maud::Markup {
    maud::html! {
        (maud::DOCTYPE)
        (render_main_app_header())
        body {
            title {
                "Home"
            }
            h1 {
                "Demo Real Estate Page"
            }
        }
    }
}
