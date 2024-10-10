use clap::Parser;
use web_builder::{
    config::{self},
    postgres,
    router::create_router,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();

    let config = config::EnvConfig::parse();

    let pg_pool = postgres::create_pool(&config);

    tracing::info!("Postgres pool created");

    postgres::migrate_up(&pg_pool).await;

    let app = create_router(pg_pool, config);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    tracing::info!("Listening on http://0.0.0.0:3000");

    axum::serve(listener, app).await.unwrap();
}
