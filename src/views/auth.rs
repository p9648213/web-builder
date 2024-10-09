use axum::response::{Html, IntoResponse};
use sailfish::Template;

#[derive(Template)]
#[template(path = "login.stpl")]
#[template(rm_whitespace = true)]
struct AuthTemplate {
    user: Vec<String>,
}

pub async fn auth() -> impl IntoResponse {
    let ctx = AuthTemplate {
        user: vec!["user".to_string(), "user2".to_string()],
    };
    Html(ctx.render().unwrap())
}
