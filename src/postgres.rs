use deadpool_postgres::{Config, Pool, Runtime};
use tokio_postgres::NoTls;
use tokio_postgres_migration::Migration;

use crate::config::EnvConfig;

const SCRIPTS_UP: [(&str, &str); 1] = [(
    "0001_create-users",
    include_str!("../migrations/0001_create-users_up.sql"),
)];

const SCRIPTS_DOWN: [(&str, &str); 1] = [(
    "0001_create-users",
    include_str!("../migrations/0001_create-users_down.sql"),
)];

fn create_config(config: &EnvConfig) -> Config {
    let mut cfg = Config::new();
    cfg.host = Some(config.pg_host.clone());
    cfg.dbname = Some(config.pg_dbname.clone());
    cfg.user = Some(config.pg_user.clone());
    cfg.password = Some(config.pg_password.clone());
    cfg
}

pub fn create_pool(config: &EnvConfig) -> Pool {
    create_config(config)
        .create_pool(Some(Runtime::Tokio1), NoTls)
        .expect("Couldn't create postgres pool")
}

pub async fn migrate_up(pool: &Pool) {
    let mut client = pool.get().await.expect("Couldn't get postgres client");
    let migration = Migration::new("migrations".to_string());
    migration
        .up(&mut **client, &SCRIPTS_UP)
        .await
        .expect("Couldn't run migrations");
}

pub async fn migrate_down(pool: &Pool) {
    let mut client = pool.get().await.expect("Couldn't get postgres client");
    let migration = Migration::new("migrations".to_string());
    migration
        .down(&mut **client, &SCRIPTS_DOWN)
        .await
        .expect("Couldn't run migrations");
}
