use axum::response::{Html, IntoResponse};
use axum_csrf::CsrfToken;
use sailfish::TemplateOnce;

use crate::utilities::minify::minify_html;

#[derive(TemplateOnce)]
#[template(path = "builder/login.stpl")]
struct BuilderLoginPage {
    authenticity_token: String,
}

pub async fn login_page(token: CsrfToken) -> impl IntoResponse {
    let ctx = BuilderLoginPage {
        authenticity_token: token.authenticity_token().unwrap(),
    };

    let html = ctx.render_once().unwrap();

    (token, Html(minify_html(&html))).into_response()
}

#[derive(TemplateOnce)]
#[template(path = "builder/register.stpl")]
struct BuilderRegisterPage {
    authenticity_token: String,
}

pub async fn register_page(token: CsrfToken) -> impl IntoResponse {
    let ctx = BuilderRegisterPage {
        authenticity_token: token.authenticity_token().unwrap(),
    };

    let html = ctx.render_once().unwrap();

    (token, Html(minify_html(&html))).into_response()
}
