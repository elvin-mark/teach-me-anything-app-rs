use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use std::{env, fs, path::Path, str::FromStr};

pub type DbPool = SqlitePool;

pub async fn init_db_pool() -> DbPool {
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:./data.db".to_string());

    if let Some(db_path) = database_url.strip_prefix("sqlite:")
        && let Some(parent) = Path::new(db_path).parent()
    {
        fs::create_dir_all(parent).expect("Failed to create database directory");
    }

    let connection_options = SqliteConnectOptions::from_str(&database_url)
        .expect("Invalid database URL")
        .create_if_missing(true);

    SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(connection_options)
        .await
        .expect("Failed to create pool")
}

pub async fn run_migrations(pool: &sqlx::SqlitePool) {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL,
            email TEXT NOT NULL UNIQUE,
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        )
        "#,
    )
    .execute(pool)
    .await
    .expect("Failed to create users table");
}
