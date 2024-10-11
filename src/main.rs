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

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", &config.port))
        .await
        .unwrap();

    let app = create_router(pg_pool, config);

    tracing::info!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
