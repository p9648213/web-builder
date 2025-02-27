use crate::{
    config::EnvConfig,
    controllers::{
        builder::{
            self,
            auth::{get_login_page, get_register_page, login, logout, register},
            choose_style::update_style,
            data::{create_data_source, update_rso_status},
            pages::{
                get_create_website_page, get_edit_page, get_select_template_page,
                get_setup_data_page,
            },
            website::{create_website, select_template_for_webiste},
        },
        real_estate::{
            self,
            data::{get_baths, get_beds, get_listing_type, get_price},
            pages::{
                get_real_estate_contact_page, get_real_estate_home_page,
                get_real_estate_property_page, get_real_estate_search_result_page,
            },
            rso_data::{
                get_hot_properties, get_locations, get_property, get_property_types,
                get_search_result,
            },
        },
    },
    middlewares::{auth::auth_middleware, csrf::csrf_middleware},
    models::state::AppState,
};
use axum::{
    http::{header, HeaderValue, StatusCode},
    middleware::from_fn_with_state,
    response::{IntoResponse, Response},
    routing::{get, patch, post},
    Router,
};
use axum_csrf::{CsrfConfig, CsrfLayer};
use axum_session::{SessionConfig, SessionLayer, SessionStore};
use axum_session_redispool::SessionRedisPool;
use deadpool_postgres::Pool;
use redis_pool::SingleRedisPool;
use tower_http::set_header::SetResponseHeaderLayer;

async fn ping() -> &'static str {
    "pong"
}

async fn fallback() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Not Found")
}

async fn builder_fallback() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::TEMPORARY_REDIRECT)
        .header(header::LOCATION, "/builder/auth/login")
        .body(axum::body::Body::empty())
        .unwrap()
}

pub async fn create_router(
    pg_pool: Pool,
    redis_pool: SingleRedisPool,
    config: EnvConfig,
) -> Router {
    let cache_control_layer = SetResponseHeaderLayer::if_not_present(
        header::CACHE_CONTROL,
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

    let builder_routes = Router::new().nest(
        "/builder",
        Router::new()
            .route("/auth/login", post(login))
            .route("/auth/register", post(register))
            .route("/auth/logout", post(logout))
            .route("/auth/login", get(get_login_page))
            .route("/auth/register", get(get_register_page))
            .route("/create-website", get(get_create_website_page))
            .route("/select-template", get(get_select_template_page))
            .route("/setup-data", get(get_setup_data_page))
            .route("/contents/{section}", get(builder::section::get_section))
            .route("/website/data/rso-data/details", post(create_data_source))
            .route("/website/data/rso-data/status", patch(update_rso_status))
            .route("/website/create", post(create_website))
            .route(
                "/website/template/select",
                post(select_template_for_webiste),
            )
            .route(
                "/edit/{website_id}/{section}/{sub_section}",
                get(get_edit_page),
            )
            .route(
                "/edit/{user_id}/{setting_id}/{part}/{theme}",
                patch(update_style),
            )
            .route("/", get(builder_fallback)),
    );

    let main_view_routes = Router::new()
        .route("/", get(get_real_estate_home_page))
        .route("/search-results", get(get_real_estate_search_result_page))
        .route("/property", get(get_real_estate_property_page))
        .route("/contact", get(get_real_estate_contact_page));

    let real_estate_rso_routes = Router::new().nest(
        "/rso",
        Router::new()
            .route("/location", get(get_locations))
            .route("/property-types", get(get_property_types))
            .route("/hot-properties", get(get_hot_properties))
            .route("/search-results", get(get_search_result))
            .route("/property", get(get_property)),
    );

    let real_estate_data_routes = Router::new().nest(
        "/data/real-estate",
        Router::new()
            .route("/listing-type", get(get_listing_type))
            .route("/prices", get(get_price))
            .route("/beds", get(get_beds))
            .route("/baths", get(get_baths)),
    );

    let real_estate_section_routes = Router::new().nest(
        "/section/real-estate",
        Router::new().route(
            "/contents/{section}",
            get(real_estate::section::get_section),
        ),
    );

    Router::new()
        .merge(builder_routes)
        .layer(from_fn_with_state(app_state.clone(), auth_middleware))
        .layer(from_fn_with_state(app_state.clone(), csrf_middleware))
        .layer(CsrfLayer::new(cfrs_config))
        .layer(SessionLayer::new(session_store))
        .merge(main_view_routes)
        .merge(real_estate_rso_routes)
        .merge(real_estate_data_routes)
        .merge(real_estate_section_routes)
        .with_state(app_state.clone())
        .layer(cache_control_layer)
        .route("/ping", get(ping))
        .fallback(fallback)
}
