use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use tokio_postgres::NoTls;
use tokio_postgres_migration::Migration;

use crate::config::EnvConfig;

const SCRIPTS_UP: [(&str, &str); 5] = [
    (
        "0001_create-users",
        include_str!("../migrations/0001_create-users_up.sql"),
    ),
    (
        "0002_insert-users",
        include_str!("../migrations/0002_insert-users_up.sql"),
    ),
    (
        "0003_create-templates",
        include_str!("../migrations/0003_create-templates_up.sql"),
    ),
    (
        "0004_create-websites",
        include_str!("../migrations/0004_create-websites_up.sql"),
    ),
    (
        "0005_insert-templates",
        include_str!("../migrations/0005_insert-templates_up.sql"),
    ),
];

const SCRIPTS_DOWN: [(&str, &str); 5] = [
    (
        "0004_create-websites",
        include_str!("../migrations/0004_create-websites_down.sql"),
    ),
    (
        "0005_insert-templates",
        include_str!("../migrations/0005_insert-templates_down.sql"),
    ),
    (
        "0003_create-templates",
        include_str!("../migrations/0003_create-templates_down.sql"),
    ),
    (
        "0002_insert-users",
        include_str!("../migrations/0002_insert-users_down.sql"),
    ),
    (
        "0001_create-users",
        include_str!("../migrations/0001_create-users_down.sql"),
    ),
];

fn create_config(config: &EnvConfig) -> tokio_postgres::Config {
    let mut cfg = tokio_postgres::Config::new();
    cfg.host(&config.pg_host);
    cfg.port(config.pg_port);
    cfg.dbname(&config.pg_dbname);
    cfg.user(&config.pg_user);
    cfg.password(&config.pg_password);
    cfg
}

pub fn create_pool(config: &EnvConfig) -> Pool {
    let pg_config = create_config(config);

    let manager_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    };

    let manager = Manager::from_config(pg_config, NoTls, manager_config);

    Pool::builder(manager)
        .max_size(16)
        .build()
        .expect("Failed to create pool")
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
