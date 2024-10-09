use crate::{config::EnvConfig, models::state::AppState, views::auth::auth};
use axum::{routing::get, Router};
use deadpool_postgres::Pool;
use maud::{html, Markup, DOCTYPE};

async fn check_health() -> &'static str {
    "pong"
}

pub fn create_router(pool: Pool, config: EnvConfig) -> Router {
    let app_state = AppState { pool, config };

    Router::new()
        .route("/check-health", get(check_health))
        .route("/auth/login", get(auth))
        .route("/auth/login2", get(login2))
        .with_state(app_state)
}

pub async fn login2() -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head{}
            body{
                h2 {"Login Page"}
                @for number in 0..50000 {
                    div {(number)}
                }
            }
        }

    }
}
