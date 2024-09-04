use sqlx::{Error, SqlitePool};



pub async fn create_tables(pool: &SqlitePool) -> Result<(), Error> {
    let query = r#"
        CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            description TEXT NOT NULL,
            status BOOLEAN NOT NULL
        );
    "#;

    sqlx::query(query)
        .execute(pool)
        .await?;

    Ok(())
}
