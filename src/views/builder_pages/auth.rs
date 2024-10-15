use axum::response::{Html, IntoResponse};
use axum_csrf::CsrfToken;
use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "builder_pages/login.stpl")]
struct BuilderLoginPage {
    authenticity_token: String,
}

pub async fn login_page(token: CsrfToken) -> impl IntoResponse {
    let ctx = BuilderLoginPage {
        authenticity_token: token.authenticity_token().unwrap(),
    };

    (token, Html(ctx.render_once().unwrap())).into_response()
}

#[derive(TemplateOnce)]
#[template(path = "builder_pages/register.stpl")]
struct BuilderRegisterPage {
    authenticity_token: String,
}

pub async fn register_page(token: CsrfToken) -> impl IntoResponse {
    let ctx = BuilderRegisterPage {
        authenticity_token: token.authenticity_token().unwrap(),
    };

    (token, Html(ctx.render_once().unwrap())).into_response()
}
