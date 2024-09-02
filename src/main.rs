use actix_web::{get, web, App, HttpServer, Responder};

use projet_web_server::common::{AppState, Task};
use projet_web_server::tasks_config::tasks_config;

#[get("/")]
async fn index(data: web::Data<AppState>) -> impl Responder {
    // HttpResponse::Ok().body("Hello world!")
    let app_name = &data.app_name;
    format!("Welcome to {}!", app_name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("ToDo List"),
                tasks: Some(vec![Task {
                    id: 1,
                    title: String::from("Test init task"),
                    description: String::from(
                        "What happens if a task is already initialized in the State?",
                    ),
                    status: false,
                }]),
            }))
            .service(index)
            .service(web::scope("/api").configure(tasks_config))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
