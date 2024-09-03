use actix_web::{web, HttpResponse, Responder};

use crate::common::{BasicResponse, SingleTaskResponse, TasksResponse};
use crate::models::{AppState, NewTask, Task};
use crate::utils::calculate_new_id;

pub async fn get_tasks(data: web::Data<AppState>) -> impl Responder {
    let tasks = data.tasks.lock().unwrap();

    let response = TasksResponse {
        status: String::from("success"),
        results: tasks.len(),
        tasks: tasks.clone(),
    };

    HttpResponse::Ok().json(response)
}

pub async fn get_task_detail(id: web::Path<u32>, data: web::Data<AppState>) -> impl Responder {
    let tasks = data.tasks.lock().unwrap();

    let task = tasks.iter().find(|t| t.id == *id);

    if task.is_none() {
        let error = BasicResponse {
            status: String::from("error"),
            message: format!("The task with ID: {} not found", id),
        };

        return HttpResponse::NotFound().json(error);
    }

    let response = SingleTaskResponse {
        status: String::from("success"),
        task: task.unwrap().clone(),
    };

    HttpResponse::Ok().json(response)
}

pub async fn add_task(body: web::Json<NewTask>, data: web::Data<AppState>) -> impl Responder {
    let mut tasks = data.tasks.lock().unwrap();

    let task = tasks.iter().find(|t| t.title == body.title);

    if task.is_some() {
        let error = BasicResponse {
            status: String::from("error"),
            message: format!("A task with title: '{}' already exists", body.title),
        };

        return HttpResponse::Conflict().json(error);
    }

    let new_task = Task {
        id: calculate_new_id(&tasks),
        title: body.title.clone(),
        description: body.description.clone(),
        status: false,
    };

    tasks.push(new_task.clone());

    let response = SingleTaskResponse {
        status: String::from("success"),
        task: new_task,
    };

    HttpResponse::Ok().json(response)
}

pub async fn edit_task(
    id: web::Path<u32>,
    body: web::Json<Task>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut tasks = data.tasks.lock().unwrap();

    let task = tasks.iter_mut().find(|t| t.id == *id);

    if task.is_none() {
        let error = BasicResponse {
            status: String::from("error"),
            message: format!("The task with ID: {} not found", body.id),
        };

        return HttpResponse::NotFound().json(error);
    }

    let task = task.unwrap();
    *task = body.clone();

    let response = SingleTaskResponse {
        status: String::from("sucess"),
        task: task.clone(),
    };

    HttpResponse::Ok().json(response)
}

pub async fn remove_task(id: web::Path<u32>, data: web::Data<AppState>) -> impl Responder {
    let mut tasks = data.tasks.lock().unwrap();

    let task = tasks.iter_mut().find(|t| t.id == *id);

    if task.is_none() {
        let error = BasicResponse {
            status: String::from("error"),
            message: format!("The task with ID: {} not found", id),
        };

        return HttpResponse::NotFound().json(error);
    }

    tasks.retain(|t| t.id != *id);

    HttpResponse::NoContent().finish()
}
