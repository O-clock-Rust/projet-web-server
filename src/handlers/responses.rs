use actix_web::HttpResponse;
use sqlx::Error;

use crate::common::{BasicResponse, SingleTaskResponse, TasksResponse};
use crate::models::Task;

pub fn fetch_all_success(tasks: Vec<Task>) -> HttpResponse {
    let response = TasksResponse {
        status: String::from("success"),
        results: tasks.len(),
        tasks: tasks.clone(),
    };

    HttpResponse::Ok().json(response)
}

pub fn fetch_one_success(task: Task) -> HttpResponse {
    let response = SingleTaskResponse {
        status: String::from("success"),
        task,
    };

    HttpResponse::Ok().json(response)
}

pub fn message_error(msg: &str) -> HttpResponse {
    let error = BasicResponse {
        status: String::from("error"),
        message: String::from(msg),
    };

    HttpResponse::InternalServerError().json(error)
}

pub fn task_fetch_error(which: &str, e: Error) -> HttpResponse {
    let error = BasicResponse {
        status: String::from("error"),
        message: format!("Failed to fetch the {} task: {}", which, e),
    };

    HttpResponse::InternalServerError().json(error)
}

pub fn task_insert_error(e: Error) -> HttpResponse {
    let error = BasicResponse {
        status: String::from("error"),
        message: format!("Failed to add task: {}", e),
    };

    HttpResponse::InternalServerError().json(error)
}

pub fn task_update_error(e: Error) -> HttpResponse {
    let error = BasicResponse {
        status: String::from("error"),
        message: format!("Failed to update the task: {}", e),
    };

    HttpResponse::InternalServerError().json(error)
}

pub fn task_delete_error(e: Error) -> HttpResponse {
    let error = BasicResponse {
        status: String::from("error"),
        message: format!("Failed to delete the task: {}", e),
    };

    HttpResponse::InternalServerError().json(error)
}

pub fn task_not_found(id: u32) -> HttpResponse {
    let error = BasicResponse {
        status: String::from("error"),
        message: format!("The task with ID: {} not found", id),
    };

    HttpResponse::NotFound().json(error)
}

pub fn task_already_exists(title: String) -> HttpResponse {
    let error = BasicResponse {
        status: String::from("error"),
        message: format!("A task with title: '{}' already exists", title),
    };

    HttpResponse::Conflict().json(error)
}
