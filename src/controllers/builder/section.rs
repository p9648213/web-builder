use axum::{
    extract::Path,
    response::{Html, IntoResponse},
};
use axum_csrf::CsrfToken;

use crate::views::builder::{data::render_setup_data, template::render_choose_template};

pub async fn get_section(Path(section): Path<String>, token: CsrfToken) -> impl IntoResponse {
    let authenticity_token = token.authenticity_token().unwrap_or("".to_owned());

    match section.as_str() {
        "template" => Html(render_choose_template().into_string()).into_response(),
        "data" => (
            token,
            Html(render_setup_data(authenticity_token).into_string()),
        )
            .into_response(),
        _ => Html("Not found".to_owned()).into_response(),
    }
}
