use crate::models::error::AppError;
use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use axum_session::Session;
use axum_session_redispool::SessionRedisPool;

pub async fn auth_middleware(
    session: Session<SessionRedisPool>,
    request: Request,
    next: Next,
) -> Result<impl IntoResponse, AppError> {
    let user_id: Option<i32> = session.get("id");

    if let Some(_) = user_id {
        match request.uri().path() {
            "/auth/login" | "/auth/register" => Ok(redirect_307("/")),
            _ => Ok(next.run(request).await.into_response()),
        }
    } else {
        let hx_current_url = request.headers().get("Hx-Current-Url");
        match hx_current_url {
            Some(hx_current_url) => {
                let url = hx_current_url.to_str().unwrap_or("");
                if url.contains("auth") {
                    Ok(next.run(request).await.into_response())
                } else {
                    Ok(Response::builder()
                        .status(StatusCode::NO_CONTENT)
                        .header("Hx-Location", "/auth/login")
                        .body(axum::body::Body::empty())
                        .unwrap())
                }
            }
            None => match request.uri().path() {
                "/auth/login" | "/auth/register" => Ok(next.run(request).await.into_response()),
                _ => Ok(redirect_307("/auth/login")),
            },
        }
    }
}

fn redirect_307(location: &str) -> Response {
    Response::builder()
        .status(StatusCode::TEMPORARY_REDIRECT)
        .header(header::LOCATION, location)
        .body(axum::body::Body::empty())
        .unwrap()
}
