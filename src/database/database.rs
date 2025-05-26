use std::str::FromStr;

use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    SqlitePool,
};

pub trait DataModel {
    async fn create_table(pool: &SqlitePool);
}

pub async fn connect_to_database() -> SqlitePool {
    let connect_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();

    SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(connect_opts)
        .await
        .expect("Connection to the database failed")
}
