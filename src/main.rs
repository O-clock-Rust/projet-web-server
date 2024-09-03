use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

use projet_web_server::common::BasicResponse;
use projet_web_server::models::AppState;
use projet_web_server::tasks_config::tasks_config;

#[get("/")]
async fn index(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    let response = BasicResponse {
        status: String::from("success"),
        message: format!("Welcome to {}!", app_name),
    };

    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let initial_state = AppState::init();
    let app_data = web::Data::new(initial_state);

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
