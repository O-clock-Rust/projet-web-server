use actix_web::web;

use crate::handlers::tasks::{add_task, edit_task, get_task_detail, get_tasks, remove_task};

pub fn tasks_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tasks")
            .service(
                web::resource("")
                    .route(web::get().to(get_tasks))
                    .route(web::post().to(add_task)),
            )
            .service(
                web::scope("/{task_id}").service(
                    web::resource("")
                        .route(web::get().to(get_task_detail))
                        .route(web::put().to(edit_task))
                        .route(web::get().to(remove_task)),
                ),
            ),
    );
}
