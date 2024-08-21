use actix_web::{App, HttpServer};
use tasks::{create_task, delete_task, edit_task, get_task, get_tasks};

mod tasks;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
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
