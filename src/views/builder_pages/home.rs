use axum::response::{Html, IntoResponse};
use axum_csrf::CsrfToken;
use axum_session::Session;
use axum_session_redispool::SessionRedisPool;
use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "builder_pages/home.stpl")]
struct BuilderHomePageTemplate {
    user_id: i32,
    authenticity_token: String,
}

pub async fn home_page(session: Session<SessionRedisPool>, token: CsrfToken) -> impl IntoResponse {
    let id: i32 = session.get("id").unwrap_or(0);

    let ctx = BuilderHomePageTemplate {
        user_id: id,
        authenticity_token: token.authenticity_token().unwrap(),
    };

    (token, Html(ctx.render_once().unwrap())).into_response()
}
