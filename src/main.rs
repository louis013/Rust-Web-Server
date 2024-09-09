mod models;
mod handlers;
mod routes;
mod app_state;

use actix_cors::Cors;
use actix_web::{App, HttpServer, web, http::header};
use app_state::AppState;
use models::Database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
     // Attempt to load the database from a file, or create a new one if loading fails
    let db = match Database::load_from_file() {
        Ok(db) => db,
        Err(_) => Database::new(),
    };

    // Wrap the database in an AppState and share it with the application
    let data = web::Data::new(AppState {
        db: std::sync::Mutex::new(db),
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
            .configure(routes::configure)  // Use the routing module
    })
    .bind("127.0.0.1:8080")? // Bind the server to the local IP address and port 8080
    .run()
    .await
}