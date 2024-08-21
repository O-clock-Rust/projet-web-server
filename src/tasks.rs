use actix_web::{delete, get, post, put, web, HttpResponse, Responder, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Task {
    id: u32,
    title: String,
    description: String,
    status: bool,
}

#[derive(Serialize, Deserialize)]
struct AddTask {
    title: String,
    description: String,
    status: bool,
}

#[post("/tasks")]
async fn create_task(task: web::Json<AddTask>) -> Result<impl Responder> {
    let task = Task {
        id: 1,
        title: task.title.clone(),
        description: task.description.clone(),
        status: task.status,
    };

    Ok(web::Json(task))
}

#[get("/tasks")]
async fn get_tasks() -> Result<impl Responder> {
    let first_task = Task {
        id: 1,
        title: "First task".to_string(),
        description: "This is the first task".to_string(),
        status: true,
    };

    let second_task = Task {
        id: 2,
        title: "Second task".to_string(),
        description: "This is the second task".to_string(),
        status: true,
    };

    Ok(web::Json([first_task, second_task]))
}

#[get("/tasks/{id}")]
async fn get_task(id: web::Path<u32>) -> Result<impl Responder> {
    let task = Task {
        id: 1,
        title: "First task".to_string(),
        description: "This is the first task".to_string(),
        status: true,
    };

    Ok(web::Json(task))
}

#[put("/tasks/{id}")]
async fn edit_task(id: web::Path<u32>, task: web::Json<AddTask>) -> Result<impl Responder> {
    let task = Task {
        id: 1,
        title: task.title.clone(),
        description: task.description.clone(),
        status: task.status,
    };

    Ok(web::Json(task))
}

#[delete("/tasks/{id}")]
async fn delete_task(id: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok()
}
