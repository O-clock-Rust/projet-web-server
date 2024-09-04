use actix_web::web;
use sqlx::{sqlite::SqliteQueryResult, Error, Pool, Sqlite};

use super::{NewTask, Task};

pub async fn find_all(pool: web::Data<Pool<Sqlite>>) -> Result<Vec<Task>, Error> {
    sqlx::query_as::<_, Task>(
        r#"
        SELECT id, title, description, status FROM tasks
        "#,
    )
    .fetch_all(pool.get_ref())
    .await
}

pub async fn find_task<T>(pool: web::Data<Pool<Sqlite>>, id: T) -> Result<Task, Error>
where
    T: Into<i64>,
{
    let id = id.into();

    sqlx::query_as::<_, Task>(
        r#"
        SELECT id, title, description, status
        FROM tasks
        WHERE id = ?
        "#,
    )
    .bind(id)
    .fetch_one(pool.get_ref())
    .await
}

pub async fn select_task(pool: web::Data<Pool<Sqlite>>, title: String) -> Result<Task, Error> {
    sqlx::query_as::<_, Task>(
        r#"
        SELECT id, title, description, status FROM tasks WHERE title = ?
        "#,
    )
    .bind(title)
    .fetch_one(pool.get_ref())
    .await
}

pub async fn insert_task(
    pool: web::Data<Pool<Sqlite>>,
    body: web::Json<NewTask>,
) -> Result<SqliteQueryResult, Error> {
    sqlx::query("INSERT INTO tasks (title, description, status) VALUES (?, ?, ?)")
        .bind(&body.title)
        .bind(&body.description)
        .bind(false)
        .execute(pool.get_ref())
        .await
}

pub async fn update_task(
    pool: web::Data<Pool<Sqlite>>,
    id: u32,
    body: web::Json<Task>,
) -> Result<SqliteQueryResult, Error> {
    sqlx::query("UPDATE tasks SET title = ?, description = ?, status = ? WHERE id = ?")
        .bind(&body.title)
        .bind(&body.description)
        .bind(body.status)
        .bind(id)
        .execute(pool.get_ref())
        .await
}

pub async fn delete_task(
    pool: web::Data<Pool<Sqlite>>,
    id: u32,
) -> Result<SqliteQueryResult, Error> {
    sqlx::query("DELETE FROM tasks WHERE id = ?")
        .bind(id)
        .execute(pool.get_ref())
        .await
}
