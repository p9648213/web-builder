use axum::{
    extract::{FromRef, State},
    routing::get,
    Router,
};
use clap::Parser;
use deadpool_postgres::Pool;
use web_builder::{
    config::{self, EnvConfig},
    postgres,
};

#[derive(Clone, FromRef)]
pub struct AppState {
    pool: Pool,
    config: EnvConfig,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();

    let config = config::EnvConfig::parse();

    let pg_pool = postgres::create_pool(&config);
    postgres::migrate_down(&pg_pool).await;

    let app_state = AppState {
        pool: pg_pool,
        config,
    };

    let app = Router::new()
        .route("/", get(check_health))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn check_health(State(pool): State<Pool>, State(config): State<EnvConfig>) -> &'static str {
    dbg!(config);
    let client = pool.get().await.unwrap();
    let stmt = client.prepare("SELECT * FROM users").await.unwrap();
    let rows = client.query(&stmt, &[]).await.unwrap();
    dbg!(rows);
    "OK"
}
