use crate::{config::EnvConfig, models::state::AppState, views::auth::login};
use axum::{
    http::{header::CACHE_CONTROL, HeaderValue},
    routing::get,
    Router,
};
use deadpool_postgres::Pool;
use tower_http::{
    compression::CompressionLayer, set_header::SetResponseHeaderLayer,
};
use rust_embed::Embed;
use axum_embed::ServeEmbed;

#[derive(Embed, Clone)]
#[folder = "assets"]
struct Assets;

async fn ping() -> &'static str {
    "pong"
}

pub fn create_router(pool: Pool, config: EnvConfig) -> Router {
    let app_state = AppState { pool, config };

    let serve_assets = ServeEmbed::<Assets>::new();

    let cache_control_layer = SetResponseHeaderLayer::if_not_present(
        CACHE_CONTROL,
        HeaderValue::from_static("no-cache, no-store, must-revalidate"),
    );

    Router::new()
        .route("/auth/login", get(login))
        .with_state(app_state)
        .layer(cache_control_layer)
        .route("/ping", get(ping))
        .nest_service("/assets", serve_assets)
        .layer(CompressionLayer::new())
}
