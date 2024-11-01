use crate::views::real_estate::header::render_main_app_header;

pub fn render_real_estate_demo_page() -> String {
    vy::render! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                {render_main_app_header()}
                <title>"Home"</title>
            </head>
            <body hx-boost="true" hx-history="false">
                <h1>"Demo Real Estate Page"</h1>
            </body>
        </html>
    }
}
