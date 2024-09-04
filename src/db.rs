use std::fs;

use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

pub async fn init_pool(database_url: &str) -> Result<Pool<Sqlite>, sqlx::Error> {
    SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
}

pub async fn create(pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    let sql = fs::read_to_string("create_table.sql")?;
    sqlx::query(&sql).execute(pool).await?;
    Ok(())
}
