use crate::{
    config::EnvConfig,
    controllers::{
        builder::{
            auth::{get_login_page, get_register_page, login, logout, register},
            data::{create_data_source, update_rso_status},
            home::get_home_page,
            rso_data::{get_listing_type, get_locations},
            section::get_section,
            website::{create_website, select_template_for_webiste},
        },
        real_estate::demo::get_real_estate_demo_page,
    },
    middlewares::{auth::auth_middleware, csrf::csrf_middleware},
    models::state::AppState,
};
use axum::{
    http::{header::CACHE_CONTROL, HeaderValue, StatusCode},
    middleware::from_fn_with_state,
    response::IntoResponse,
    routing::{get, patch, post},
    Router,
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

    let cfrs_config = CsrfConfig::default().with_key(Some(
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
        .route("/auth/login", post(login))
        .route("/auth/register", post(register))
        .route("/auth/logout", post(logout))
        .route("/auth/login", get(get_login_page))
        .route("/auth/register", get(get_register_page))
        .route("/", get(get_home_page))
        .route("/contents/:section", get(get_section))
        .route("/website/data/rso-data/details", post(create_data_source))
        .route("/website/data/rso-data/status", patch(update_rso_status))
        .route("/website/create", post(create_website))
        .route(
            "/website/template/select",
            post(select_template_for_webiste),
        );

    let demo_routes = Router::new().route("/realestate", get(get_real_estate_demo_page));

    let rso_routes = Router::new()
        .route("/listing-type", get(get_listing_type))
        .route("/location", get(get_locations));

    Router::new()
        .nest("/builder", builder_routes)
        .nest("/demo", demo_routes)
        .layer(from_fn_with_state(app_state.clone(), auth_middleware))
        .nest("/rso", rso_routes)
        .with_state(app_state.clone())
        .layer(cache_control_layer)
        .layer(SessionLayer::new(session_store))
        .layer(from_fn_with_state(app_state.clone(), csrf_middleware))
        .layer(CsrfLayer::new(cfrs_config))
        .route("/ping", get(ping))
        .nest_service("/assets", serve_assets)
        .layer(CompressionLayer::new())
        .fallback(fallback)
}
