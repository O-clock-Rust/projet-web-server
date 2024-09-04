use std::env;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;

use projet_web_server::common::BasicResponse;
use projet_web_server::db;
use projet_web_server::tasks_config::tasks_config;

#[get("/")]
async fn index() -> impl Responder {
    let response = BasicResponse {
        status: String::from("success"),
        message: String::from("Welcome to ToDo List API!"),
    };

    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = db::init_pool(&database_url)
        .await
        .expect("Failed to initialize Database");

    db::create(&pool).await.expect("Failes to create table");

    let app_data = web::Data::new(pool.clone());

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(index)
            .service(web::scope("/api").configure(tasks_config))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
