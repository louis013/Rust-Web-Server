use crate::models::Database;
use std::sync::Mutex;

// The application's state, shared across all HTTP requests
pub struct AppState {
   pub db: Mutex<Database> // Database is wrapped in a Mutex for safe concurrent access
}