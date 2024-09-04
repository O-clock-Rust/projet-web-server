use std::env;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::sqlite::SqlitePoolOptions;

use tasks::{create_task, delete_task, edit_task, get_task, get_tasks};

mod db;
mod tasks;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url).await.expect("Failed to connect to the database");

    db::create_tables(&pool).await.expect("Failed to cretae tables");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(create_task)
            .service(get_tasks)
            .service(get_task)
            .service(edit_task)
            .service(delete_task)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
