use crate::{
    config::EnvConfig,
    handlers::auth::{login, register},
    middlewares::{auth::auth_middleware, csrf::csrf_middleware},
    models::state::AppState,
    views::pages::{
        auth::{login_page, register_page},
        home::home_page,
    },
};
use axum::{
    http::{header::CACHE_CONTROL, HeaderValue},
    middleware::from_fn_with_state,
    routing::{get, post},
    Router,
};
use axum_csrf::{CsrfConfig, CsrfLayer, Key};
use axum_embed::ServeEmbed;
use deadpool_postgres::Pool;
use rust_embed::Embed;
use tower_http::{compression::CompressionLayer, set_header::SetResponseHeaderLayer};

#[derive(Embed, Clone)]
#[folder = "assets"]
struct Assets;

async fn ping() -> &'static str {
    "pong"
}

pub fn create_router(pool: Pool, config: EnvConfig) -> Router {
    let serve_assets = ServeEmbed::<Assets>::new();

    let cache_control_layer = SetResponseHeaderLayer::if_not_present(
        CACHE_CONTROL,
        HeaderValue::from_static("no-cache, no-store, must-revalidate"),
    );

    let cfrs_confir = CsrfConfig::default()
        .with_cookie_same_site(cookie::SameSite::Strict);

    let app_state = AppState { pool, config };

    Router::new()
        .route("/auth/login", post(login))
        .route("/auth/register", post(register))
        .route("/auth/login", get(login_page))
        .route("/auth/register", get(register_page))
        .route("/", get(home_page))
        .with_state(app_state.clone())
        .layer(cache_control_layer)
        .layer(from_fn_with_state(app_state.clone(), auth_middleware))
        .layer(from_fn_with_state(app_state.clone(), csrf_middleware))
        .layer(CsrfLayer::new(cfrs_confir))
        .route("/ping", get(ping))
        .nest_service("/assets", serve_assets)
        .layer(CompressionLayer::new())
}
