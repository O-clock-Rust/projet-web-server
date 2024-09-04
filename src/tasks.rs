use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Pool, Sqlite};

#[derive(Serialize, Deserialize, FromRow)]
struct Task {
    id: i64,
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

#[derive(Serialize)]
struct JsonResponse {
    status: String,
    message: String,
}

#[post("/tasks")]
async fn create_task(pool: web::Data<Pool<Sqlite>>, task: web::Json<AddTask>) -> impl Responder {
    let result = sqlx::query(
        r#"
        INSERT INTO tasks (title, description, status) 
        VALUES (?, ?, ?)
        "#,
    )
    .bind(task.title.clone())
    .bind(task.description.clone())
    .bind(task.status)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(insert_result) => {
            let task_id = insert_result.last_insert_rowid();

            let created_task = Task {
                id: task_id,
                title: task.title.clone(),
                description: task.description.clone(),
                status: task.status,
            };

            HttpResponse::Created().json(created_task)
        },
        Err(e) => HttpResponse::InternalServerError().json(JsonResponse {
            status: "error".to_string(),
            message: format!("Failed to create the task: {}", e)
        }),
    }
}

#[get("/tasks")]
async fn get_tasks(pool: web::Data<Pool<Sqlite>>) -> impl Responder {
    let result = sqlx::query_as::<_, Task>(
        r#"
        SELECT id, title, description, status FROM tasks
        "#,
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(tasks) => HttpResponse::Ok().json(tasks),
        Err(e) => HttpResponse::InternalServerError().json(JsonResponse {
            status: "error".to_string(),
            message: format!("Failed to fetch the tasks: {}", e)
        })
    }
}

#[get("/tasks/{id}")]
async fn get_task(pool: web::Data<Pool<Sqlite>>, id: web::Path<i64>) -> impl Responder {
    let id = id.into_inner();

    let result = sqlx::query_as::<_, Task>(
        r#"
        SELECT id, title, description, status
        FROM tasks
        WHERE id = ?
        "#,
    )
    .bind(id)
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(task) => HttpResponse::Ok().json(task),
        Err(e) => HttpResponse::InternalServerError().json(JsonResponse {
            status: "error".to_string(),
            message: format!("Failed to fetch the task with the ID {}: {}", id, e)
        })
    }
}

#[put("/tasks/{id}")]
async fn edit_task(pool: web::Data<Pool<Sqlite>>, id: web::Path<i64>, task: web::Json<AddTask>) -> impl Responder {
    let task_id = id.into_inner();

    let existing_task = sqlx::query!(
        "SELECT id FROM tasks WHERE id = ?",
        task_id
    )
    .fetch_optional(pool.get_ref())
    .await;

    match existing_task {
        Ok(Some(_)) => {
            let result = sqlx::query!(
                r#"
                UPDATE tasks 
                SET title = ?, description = ?, status = ? 
                WHERE id = ?
                "#,
                task.title,
                task.description,
                task.status,
                task_id
            )
            .execute(pool.get_ref())
            .await;

            match result {
                Ok(_) => {
                    let updated_task = sqlx::query_as!(
                        Task,
                        r#"
                        SELECT id, title, description, status
                        FROM tasks
                        WHERE id = ?
                        "#,
                        task_id
                    )
                    .fetch_one(pool.get_ref())
                    .await;

                    match updated_task {
                        Ok(task) => HttpResponse::Ok().json(task),
                        Err(e) => HttpResponse::InternalServerError().json(JsonResponse {
                            status: "error".to_string(),
                            message: format!("Failed to fetch the updated task: {}", e)
                        })
                    }
                },
                Err(e) => HttpResponse::InternalServerError().json(JsonResponse {
                    status: "error".to_string(),
                    message: format!("Failed to update the task with the ID {}: {}", task_id, e)
                })
            }
        },
        Ok(None) => HttpResponse::NotFound().json(JsonResponse {
            status: "error".to_string(),
            message: format!("The task with ID: {} not found", task_id)
        }),
        Err(e) => HttpResponse::InternalServerError().json(JsonResponse {
            status: "error".to_string(),
            message: format!("Failed to fetch the task with the ID {}: {}", task_id, e)
        })
    }
}

#[delete("/tasks/{id}")]
async fn delete_task(pool: web::Data<Pool<Sqlite>>, id: web::Path<i64>) -> impl Responder {
    let task_id = id.into_inner();

    // Vérifier si la tâche existe
    let existing_task = sqlx::query!(
        "SELECT id FROM tasks WHERE id = ?",
        task_id
    )
    .fetch_optional(pool.get_ref())
    .await;

    match existing_task {
        Ok(Some(_)) => {
            let result = sqlx::query!(
                "DELETE FROM tasks WHERE id = ?",
                task_id
            )
            .execute(pool.get_ref())
            .await;

            match result {
                Ok(_) => HttpResponse::NoContent().finish(),
                Err(e) => HttpResponse::InternalServerError().json(JsonResponse {
                    status: "error".to_string(),
                    message: format!("Failed to delete the task: {}", e)
                }),
            }
        }
        Ok(None) => HttpResponse::NotFound().json(JsonResponse {
            status: "error".to_string(),
            message: format!("The task with ID: {} not found", task_id)
        }),
        Err(e) => HttpResponse::InternalServerError().json(JsonResponse {
            status: "error".to_string(),
            message: format!("Failed to fetch the task with the ID {}: {}", task_id, e)
        })
    }
}
