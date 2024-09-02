use actix_web::web;

use crate::handlers::tasks;

pub fn tasks_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tasks").service(
            web::resource("")
                .route(web::get().to(tasks::tasks_get))
                .route(web::post().to(tasks::add_task)),
        ),
    );
}
