use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/tasks")]
async fn tasks_get() -> impl Responder {
    HttpResponse::Ok().body("Tasks")
}

#[post("/tasks")]
async fn tasks_post(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
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
