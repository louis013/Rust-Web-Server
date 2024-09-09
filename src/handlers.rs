use actix_web::{ web, Responder, HttpResponse };
use crate::models::{ Task, User };
use crate::app_state::AppState;


// Handler to create a task
pub async fn create_task(app_state: web::Data<AppState>, task: web::Json<Task>) -> impl Responder {
    let mut db = app_state.db.lock().unwrap(); // Acquire a lock on the database to safely access it in a multi-threaded environment

    db.insert(task.into_inner()); // Insert the new task
    let _ = db.save_to_file(); // Save the database to a file
    HttpResponse::Ok().finish() // Respond with a success status
}

// Handler to retrieve specific task
pub async fn read_task(app_state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
    let db = app_state.db.lock().unwrap(); // Acquire a lock on the database to safely access it in a multi-threaded environment

    // Attempt to retrieve the task with the provided `id`
    match db.get(&id.into_inner()) {
        // If the task is found, respond with HTTP 200 OK and the task serialized as JSON
        Some(task) => HttpResponse::Ok().json(task),

        // If the task is not found, respond with HTTP 404 Not Found
        None => HttpResponse::NotFound().finish()
    }
}

// Handler or retrieving all tasks
pub async fn read_all_tasks(app_state: web::Data<AppState>) -> impl Responder {
    let db = app_state.db.lock().unwrap(); // Acquire a lock on the database to safely access it in a multi-threaded environment

    let tasks = db.get_all();
    HttpResponse::Ok().json(tasks)
}

// Handler for updating task
pub async fn update_task(app_state: web::Data<AppState>, task: web::Json<Task>) -> impl Responder {
    let mut db = app_state.db.lock().unwrap(); // Acquire a lock on the database to safely access it in a multi-threaded environment

    db.update(task.into_inner()); // Update the task
    let _ = db.save_to_file(); // Save the database to a file
    HttpResponse::Ok().finish() // Respond with a success status
}

// Handler for deleting specific task
pub async fn delete_task(app_state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
    let mut db = app_state.db.lock().unwrap(); // Acquire a lock on the database to safely access it in a multi-threaded environment

    db.delete(&id.into_inner()); // Delete the task
    let _ = db.save_to_file(); // Save the database to a file// Save the database to a file
    HttpResponse::Ok().finish() // Respond with a success status
}

// Handler for registering user
pub async fn register(app_state: web::Data<AppState>, user: web::Json<User>) -> impl Responder {
    let mut db = app_state.db.lock().unwrap(); // Acquire a lock on the database to safely access it in a multi-threaded environment

    db.insert_user(user.into_inner()); // Insert the new user into the database
    let _ = db.save_to_file(); // Save the updated database to a file to persist the user
    HttpResponse::Ok().finish() // Respond with HTTP 200 OK status
}

// Handler for logging in user
pub async fn login(app_state: web::Data<AppState>, user: web::Json<User>) -> impl Responder {
    let db = app_state.db.lock().unwrap(); // Acquire a lock on the database to safely access it in a multi-threaded environment

    // Attempt to retrieve the user by username
    match db.get_user_by_name(&user.username) {
        // Check if the stored user's password matches the provided password
        Some(stored_user) if stored_user.password == user.password => {
            HttpResponse::Ok().body("Logged in!") // Respond with a success message if credentials are correct
        },
        // If username or password is incorrect, respond with an error message
        _ => HttpResponse::BadRequest().body("Invalid username or password")
    }
}