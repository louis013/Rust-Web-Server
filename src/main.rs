use actix_cors::Cors;

use actix_web::{ http::header, web, App, HttpServer, Responder, HttpResponse };

use serde::{ Deserialize, Serialize };

use reqwest::Client as HttpClient;

use async_trait::async_trait;

use std::sync::Mutex;
use std::collections::HashMap;
use std::fs;
use std::io::Write;

// Define a structure representing a User
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    id: u64,
    name: String,
    completed: bool
}

// Define a structure representing a database with Tasks and Users
#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    id: u64, 
    username: String,
    password: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Database {
    tasks: HashMap<u64, Task>, // Stores tasks, keyed by their id
    users: HashMap<u64, User> // Stores users, keyed by their id
}

impl Database {
    // Create a new, empty database
    fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            users: HashMap::new()
        }
    }

    // CRUD DATA
    // Insert a task into the database
    fn insert(&mut self, task: Task) {
        self.tasks.insert(task.id, task);
    }

    // Retrieve a task by its id
    fn get(&self, id: &u64) -> Option<&Task> {
        self.tasks.get(id)
    }

    // Retrieve all tasks
    fn get_all(&self) -> Vec<&Task> {
        self.tasks.values().collect()
    }

    // Delete a task by its id
    fn delete(&mut self, id: &u64) {
        self.tasks.remove(id);
    }

    // Update a task in the database
    fn update(&mut self, task: Task) {
        self.tasks.insert(task.id, task);
    }

    // USER DATA RELATED FUNCTIONS
    // Insert a user into the database
    fn insert_user(&mut self, user: User) {
        self.users.insert(user.id, user);
    }

    // Retrieve a user by their username
    fn get_user_by_name(&self, username: &str) -> Option<&User> {
        self.users.values().find(|u| u.username == username)
    }

    // DATABASE SAVING
    // Save the database to a file in JSON format
    fn save_to_file(&self) -> std::io::Result<()> {
        let data = serde_json::to_string(&self)?;
        let mut file = fs::File::create("database.json")?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }

    // Load the database from a JSON file
    fn load_from_file() -> std::io::Result<Self> {{
        let file_content = fs::read_to_string("database.json")?;
        let db: Database = serde_json::from_str(&file_content)?;
        Ok(db)
    }}

}

// The application's state, shared across all HTTP requests
struct AppState {
    db: Mutex<Database> // Database is wrapped in a Mutex for safe concurrent access
}

// Handler to create a task
async fn create_task(app_state: web::Data<AppState>, task: web::Json<Task>) -> impl Responder {
    let mut db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap(); // Acquire a lock on the database to safely access it in a multi-threaded environment

    db.insert(task.into_inner()); // Insert the new task
    let _ = db.save_to_file(); // Save the database to a file
    HttpResponse::Ok().finish() // Respond with a success status
}

// Handler to retrieve specific task
async fn read_task(app_state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
    let db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap(); // Acquire a lock on the database to safely access it in a multi-threaded environment

    // Attempt to retrieve the task with the provided `id`
    match db.get(&id.into_inner()) {
        // If the task is found, respond with HTTP 200 OK and the task serialized as JSON
        Some(task) => HttpResponse::Ok().json(task),

        // If the task is not found, respond with HTTP 404 Not Found
        None => HttpResponse::NotFound().finish()
    }
}

// Handler or retrieving all tasks
async fn read_all_tasks(app_state: web::Data<AppState>) -> impl Responder {
    let db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap(); // Acquire a lock on the database to safely access it in a multi-threaded environment

    let tasks = db.get_all();
    HttpResponse::Ok().json(tasks)
}

// Handler for updating task
async fn update_task(app_state: web::Data<AppState>, task: web::Json<Task>) -> impl Responder {
    let mut db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap(); // Acquire a lock on the database to safely access it in a multi-threaded environment

    db.update(task.into_inner()); // Update the task
    let _ = db.save_to_file(); // Save the database to a file
    HttpResponse::Ok().finish() // Respond with a success status
}

// Handler for deleting specific task
async fn delete_task(app_state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
    let mut db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap(); // Acquire a lock on the database to safely access it in a multi-threaded environment

    db.delete(&id.into_inner()); // Delete the task
    let _ = db.save_to_file(); // Save the database to a file// Save the database to a file
    HttpResponse::Ok().finish() // Respond with a success status
}

// Handler for registering user
async fn register(app_state: web::Data<AppState>, user: web::Json<User>) -> impl Responder {
    let mut db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap(); // Acquire a lock on the database to safely access it in a multi-threaded environment

    db.insert_user(user.into_inner()); // Insert the new user into the database
    let _ = db.save_to_file(); // Save the updated database to a file to persist the user
    HttpResponse::Ok().finish() // Respond with HTTP 200 OK status
}

// Handler for logging in user
async fn login(app_state: web::Data<AppState>, user: web::Json<User>) -> impl Responder {
    let db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap(); // Acquire a lock on the database to safely access it in a multi-threaded environment

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // Attempt to load the database from a file, or create a new one if loading fails
    let db: Database = match Database::load_from_file() {
        Ok(db) => db,
        Err(_) => Database::new()
    };

    // Wrap the database in an AppState and share it with the application
    let data: web::Data<AppState> = web::Data::new(AppState {
        db: Mutex::new(db)
    });

    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::permissive() // Enable permissive CORS (Cross-Origin Resource Sharing)
                    .allowed_origin_fn(|origin, _req_head| {
                        origin.as_bytes().starts_with(b"http://localhost") || origin == "null"
                    })
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600)
            )
            .app_data(data.clone()) // Share the application state with handlers
            .route("/task", web::post().to(create_task)) // Define an endpoint for creating tasks
            .route("/task/{id}", web::get().to(read_task)) // Define an endpoint for retrieving specific task
            .route("/task", web::get().to(read_all_tasks)) // Define an endpoint for retrieving all tasks
            .route("/task", web::put().to(update_task)) // Define an endpoint for updating specific task
            .route("/task/{id}", web::delete().to(delete_task)) // Define an endpoint for deleting specific task
            .route("/register", web::post().to(register)) // Define an endpoint for user registration
            .route("/login", web::post().to(login)) // Define an enpoit for user login

    })
    .bind("127.0.0.1:8080")? // Bind the server to the local IP address and port 8080
    .run() // Run the server
    .await

}
