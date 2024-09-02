use actix_web::{web, HttpResponse, Responder};

use crate::common::AppState;

pub async fn tasks_get(data: web::Data<AppState>) -> impl Responder {
    // HttpResponse::Ok().body("Tasks")
    match &data.tasks {
        Some(tasks) => format!("The tasks:\n {:#?}", tasks),
        None => "No tasks at this moment.".to_string(),
    }
}

pub async fn add_task(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
