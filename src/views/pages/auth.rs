use axum::response::{Html, IntoResponse};
use axum_csrf::CsrfToken;
use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "pages/login.stpl")]
struct LoginPageTemplate {
    authenticity_token: String,
}

pub async fn login_page(token: CsrfToken) -> impl IntoResponse {
    let ctx = LoginPageTemplate {
        authenticity_token: token.authenticity_token().unwrap(),
    };

    (token, Html(ctx.render_once().unwrap())).into_response()
}

#[derive(TemplateOnce)]
#[template(path = "pages/register.stpl")]
struct RegisterPageTemplate {
    authenticity_token: String,
}

pub async fn register_page(token: CsrfToken) -> impl IntoResponse {
    let ctx = RegisterPageTemplate {
        authenticity_token: token.authenticity_token().unwrap(),
    };

    (token, Html(ctx.render_once().unwrap())).into_response()
}
