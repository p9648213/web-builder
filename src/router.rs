use crate::{
    config::EnvConfig,
    controllers::auth::{login, register},
    middlewares::{auth::auth_middleware, csrf::csrf_middleware},
    models::{state::AppState, user::User},
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
use axum_csrf::{CsrfConfig, CsrfLayer};
use axum_embed::ServeEmbed;
use axum_session::{SessionConfig, SessionLayer, SessionStore};
use axum_session_auth::{AuthConfig, AuthSessionLayer};
use axum_session_redispool::SessionRedisPool;
use deadpool_postgres::Pool;
use redis::{aio::MultiplexedConnection, Client};
use redis_pool::RedisPool;
use rust_embed::Embed;
use tower_http::{compression::CompressionLayer, set_header::SetResponseHeaderLayer};

#[derive(Embed, Clone)]
#[folder = "assets"]
struct Assets;

async fn ping() -> &'static str {
    "pong"
}

pub async fn create_router(pg_pool: Pool, redis_pool: RedisPool<Client, MultiplexedConnection> , config: EnvConfig) -> Router {
    let serve_assets = ServeEmbed::<Assets>::new();

    let cache_control_layer = SetResponseHeaderLayer::if_not_present(
        CACHE_CONTROL,
        HeaderValue::from_static("no-cache, no-store, must-revalidate"),
    );

    let cfrs_confir = CsrfConfig::default();

    let session_config = SessionConfig::default();

    let auth_config = AuthConfig::<i64>::default();

    let session_store = SessionStore::<SessionRedisPool>::new(Some(redis_pool.clone().into()), session_config).await.expect("Error while creating session store");

    let app_state = AppState { pg_pool, config };

    Router::new()
        .route("/auth/login", post(login))
        .route("/auth/register", post(register))
        .route("/auth/login", get(login_page))
        .route("/auth/register", get(register_page))
        .route("/", get(home_page))
        .with_state(app_state.clone())
        .layer(SessionLayer::new(session_store))
        .layer(cache_control_layer)
        .layer(from_fn_with_state(app_state.clone(), auth_middleware))
        .layer(from_fn_with_state(app_state.clone(), csrf_middleware))
        .layer(CsrfLayer::new(cfrs_confir))
        .route("/ping", get(ping))
        .nest_service("/assets", serve_assets)
        .layer(CompressionLayer::new())
}
