use axum::{extract::Path, response::Html};

use crate::views::builder::{data::render_setup_data, template::render_choose_template};

pub async fn get_section(Path(section): Path<String>) -> Html<String> {
    match section.as_str() {
        "template" => Html(render_choose_template()),
        "data" => Html(render_setup_data()),
        _ => Html("Not found".to_owned()),
    }
}
