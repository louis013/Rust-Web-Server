use serde::{ Deserialize, Serialize };
use std::collections::HashMap;
use std::fs;
use std::io::Write;

// Define a structure representing a User
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: u64,
    pub name: String,
    pub completed: bool
}

// Define a structure representing a database with Tasks and Users
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: u64, 
    pub username: String,
    pub password: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Database {
    pub tasks: HashMap<u64, Task>, // Stores tasks, keyed by their id
    pub users: HashMap<u64, User> // Stores users, keyed by their id
}

impl Database {
    // Create a new, empty database
    pub fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            users: HashMap::new()
        }
    }

    // CRUD DATA
    // Insert a task into the database
    pub fn insert(&mut self, task: Task) {
        self.tasks.insert(task.id, task);
    }

    // Retrieve a task by its id
    pub fn get(&self, id: &u64) -> Option<&Task> {
        self.tasks.get(id)
    }

    // Retrieve all tasks
    pub fn get_all(&self) -> Vec<&Task> {
        self.tasks.values().collect()
    }

    // Delete a task by its id
    pub fn delete(&mut self, id: &u64) {
        self.tasks.remove(id);
    }

    // Update a task in the database
    pub fn update(&mut self, task: Task) {
        self.tasks.insert(task.id, task);
    }

    // USER DATA RELATED FUNCTIONS
    // Insert a user into the database
    pub fn insert_user(&mut self, user: User) {
        self.users.insert(user.id, user);
    }

    // Retrieve a user by their username
    pub fn get_user_by_name(&self, username: &str) -> Option<&User> {
        self.users.values().find(|u| u.username == username)
    }

    // DATABASE SAVING
    // Save the database to a file in JSON format
    pub fn save_to_file(&self) -> std::io::Result<()> {
        let data = serde_json::to_string(&self)?;
        let mut file = fs::File::create("database.json")?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }

    // Load the database from a JSON file
    pub fn load_from_file() -> std::io::Result<Self> {{
        let file_content = fs::read_to_string("database.json")?;
        let db: Database = serde_json::from_str(&file_content)?;
        Ok(db)
    }}

}