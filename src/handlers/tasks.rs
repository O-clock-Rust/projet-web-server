use actix_web::{web, HttpResponse, Responder};
use sqlx::{Pool, Sqlite};

use crate::handlers::responses;
use crate::models::queries::{
    delete_task, find_all, find_task, insert_task, select_task, update_task,
};
use crate::models::{NewTask, Task};

pub async fn get_tasks(pool: web::Data<Pool<Sqlite>>) -> impl Responder {
    let tasks: Vec<Task> = find_all(pool).await.expect("Failed to fetch tasks");
    responses::fetch_all_success(tasks)
}

pub async fn get_task_detail(
    pool: web::Data<Pool<Sqlite>>,
    task_id: web::Path<u32>,
) -> impl Responder {
    let id = task_id.into_inner();
    let task = find_task(pool, id).await;

    if task.is_err() {
        return responses::task_not_found(id);
    }

    responses::fetch_one_success(task.unwrap())
}

pub async fn add_task(pool: web::Data<Pool<Sqlite>>, body: web::Json<NewTask>) -> impl Responder {
    let task = select_task(pool.clone(), body.title.clone()).await;

    if task.is_ok() {
        return responses::task_already_exists(body.title.clone());
    }

    let result = insert_task(pool.clone(), body).await;

    match result {
        Ok(insert_result) => {
            let task_id = insert_result.last_insert_rowid();
            let inserted_task = find_task(pool.clone(), task_id).await;

            match inserted_task {
                Ok(task) => responses::fetch_one_success(task),
                Err(e) => responses::task_insert_error(e),
            }
        }
        Err(e) => responses::task_fetch_error("newly created", e),
    }
}

pub async fn edit_task(
    pool: web::Data<Pool<Sqlite>>,
    id: web::Path<u32>,
    body: web::Json<Task>,
) -> impl Responder {
    let task_id = id.into_inner();
    let task = find_task(pool.clone(), task_id).await;

    if task.is_err() {
        return responses::task_not_found(task_id);
    }

    let result = update_task(pool.clone(), task_id, body).await;

    match result {
        Ok(update_result) => {
            if update_result.rows_affected() == 0 {
                return responses::message_error("Failed to update the task");
            }

            let updated_task = find_task(pool, task_id).await;

            match updated_task {
                Ok(task) => responses::fetch_one_success(task),
                Err(e) => responses::task_fetch_error("updated", e),
            }
        }
        Err(e) => responses::task_update_error(e),
    }
}

pub async fn remove_task(pool: web::Data<Pool<Sqlite>>, id: web::Path<u32>) -> impl Responder {
    let task_id = id.into_inner();
    let task = find_task(pool.clone(), task_id).await;

    if task.is_err() {
        return responses::task_not_found(task_id);
    }

    let result = delete_task(pool.clone(), task_id).await;

    match result {
        Ok(delete_result) => {
            if delete_result.rows_affected() == 0 {
                return responses::message_error("Failed to delete the task");
            }

            HttpResponse::NoContent().finish()
        }
        Err(e) => responses::task_delete_error(e),
    }
}
