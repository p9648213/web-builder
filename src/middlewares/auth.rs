use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use axum_extra::extract::CookieJar;
use cookie::Cookie;

use crate::{config::EnvConfig, models::error::AppError, utilities::jwt::validate_token};

pub async fn auth_middleware(
    State(config): State<EnvConfig>,
    request: Request,
    next: Next,
) -> Result<impl IntoResponse, AppError> {
    let cookie_extract = request.headers().get("cookie");

    if let Some(cookie) = cookie_extract {
        let cookie_str = cookie.to_str().map_err(|error| {
            tracing::error!("Failed to get token from cookie: {}", error);
            AppError::new(StatusCode::UNAUTHORIZED, "Invalid token".to_string())
        })?;

        let token = cookie_str.split("; ").find(|&x| x.starts_with("token="));

        if let Some(token_extract) = token {
            let token_value = &token_extract[6..token_extract.chars().count()];

            let claims = validate_token(&config.jwt_secret, token_value);

            if let Some(_) = claims {
                match request.uri().path() {
                    "/auth/login" | "/auth/register" => Ok(redirect_307("/")),
                    _ => Ok(next.run(request).await.into_response()),
                }
            } else {
                let token_cookie: Cookie = Cookie::build(("token", ""))
                    .same_site(cookie::SameSite::Strict)
                    .http_only(true)
                    .path("/")
                    .max_age(cookie::time::Duration::minutes(0))
                    .into();
                let cookies = CookieJar::new().add(token_cookie);
                Ok((cookies, redirect_307("/auth/login")).into_response())
            }
        } else {
            match request.uri().path() {
                "/auth/login" | "/auth/register" => Ok(next.run(request).await.into_response()),
                _ => Ok(redirect_307("/auth/login")),
            }
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
