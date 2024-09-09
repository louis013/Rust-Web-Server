use actix_web::web;
use crate::handlers::*;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/task", web::post().to(create_task)) // Define an endpoint for creating tasks
        .route("/task/{id}", web::get().to(read_task)) // Define an endpoint for retrieving specific task
        .route("/task", web::get().to(read_all_tasks)) // Define an endpoint for retrieving all tasks
        .route("/task", web::put().to(update_task)) // Define an endpoint for updating specific task
        .route("/task/{id}", web::delete().to(delete_task)) // Define an endpoint for deleting specific task
        .route("/register", web::post().to(register)) // Define an endpoint for user registration
        .route("/login", web::post().to(login)); // Define an endpoit for user login
}