use actix_web::{web, HttpResponse, Responder};

use crate::common::AppState;

pub async fn get_tasks(data: web::Data<AppState>) -> impl Responder {
    // HttpResponse::Ok().body("Tasks")
    match &data.tasks {
        Some(tasks) => format!("The tasks:\n {:#?}", tasks),
        None => "No tasks at this moment.".to_string(),
    }
}

pub async fn get_task_detail(id: web::Path<u32>) -> impl Responder {
    format!("Task #{}", id)
}

pub async fn add_task(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn edit_task(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn remove_task(id: web::Path<u32>) -> impl Responder {
    format!("Task #{} was deleted", id)
}
