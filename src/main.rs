use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[derive(Debug)]
struct Task {
    id: u32,
    title: String,
    description: String,
    status: bool,
}

struct AppState {
    app_name: String,
    tasks: Option<Vec<Task>>,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> impl Responder {
    // HttpResponse::Ok().body("Hello world!")
    let app_name = &data.app_name;
    format!("Welcome to {}!", app_name)
}

#[get("/tasks")]
async fn tasks_get(data: web::Data<AppState>) -> impl Responder {
    // HttpResponse::Ok().body("Tasks")
    match &data.tasks {
        Some(tasks) => format!("The tasks:\n {:#?}", tasks),
        None => "No tasks at this moment.".to_string(),
    }
}

#[post("/tasks")]
async fn tasks_post(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("ToDo List"),
                tasks: None,
            }))
            .service(
                web::scope("/api")
                    .service(index)
                    .service(tasks_get)
                    .service(tasks_post),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
