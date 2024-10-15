use crate::{
    config::EnvConfig,
    controllers::builder,
    middlewares::{auth::auth_middleware, csrf::csrf_middleware},
    models::state::AppState,
    views::{app_pages, builder_pages},
};
use axum::{
    http::{header::CACHE_CONTROL, HeaderValue, StatusCode}, middleware::from_fn_with_state, response::IntoResponse, routing::{get, post}, Router
};
use axum_csrf::{CsrfConfig, CsrfLayer};
use axum_embed::ServeEmbed;
use axum_session::{SessionConfig, SessionLayer, SessionStore};
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

async fn fallback() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Not Found")
}

pub async fn create_router(
    pg_pool: Pool,
    redis_pool: RedisPool<Client, MultiplexedConnection>,
    config: EnvConfig,
) -> Router {
    let serve_assets = ServeEmbed::<Assets>::new();

    let cache_control_layer = SetResponseHeaderLayer::if_not_present(
        CACHE_CONTROL,
        HeaderValue::from_static("no-cache, no-store, must-revalidate"),
    );

    let cfrs_key = config.csrf_encrypt_key.as_bytes();

    let session_key = config.session_encrypt_key.as_bytes();

    let database_key = config.database_encrypt_key.as_bytes();

    let cfrs_confir = CsrfConfig::default().with_key(Some(
        axum_csrf::Key::try_from(cfrs_key).expect("Error while creating csrf key"),
    ));

    let session_config = SessionConfig::default()
        .with_key(
            axum_session::Key::try_from(session_key).expect("Error while creating session key"),
        )
        .with_database_key(
            axum_session::Key::try_from(database_key).expect("Error while creating session key"),
        );

    let session_store =
        SessionStore::<SessionRedisPool>::new(Some(redis_pool.clone().into()), session_config)
            .await
            .expect("Error while creating session store");

    let app_state = AppState { pg_pool, config };

    let builder_routes = Router::new()
        .route("/auth/login", post(builder::auth::login))
        .route("/auth/register", post(builder::auth::register))
        .route("/auth/login", get(builder_pages::auth::login_page))
        .route("/auth/register", get(builder_pages::auth::register_page))
        .route("/auth/logout", post(builder::auth::logout))
        .route("/", get(builder_pages::home::home_page));

    let app_routes = Router::new().route("/", get(app_pages::home::home_page));

    Router::new()
        .nest("/builder", builder_routes)
        .with_state(app_state.clone())
        .layer(cache_control_layer)
        .layer(from_fn_with_state(app_state.clone(), auth_middleware))
        .layer(SessionLayer::new(session_store))
        .layer(from_fn_with_state(app_state.clone(), csrf_middleware))
        .layer(CsrfLayer::new(cfrs_confir))
        .nest("/app", app_routes)
        .route("/ping", get(ping))
        .nest_service("/assets", serve_assets)
        .layer(CompressionLayer::new())
        .fallback(fallback)
}
