use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    description: String,
    complete: bool,
    created_at: DateTime<Local>,
    completed_at: Option<DateTime<Local>>,
}

impl Todo {
    pub fn new(description: String, complete: Option<bool>) -> Self {
        let created_at = Local::now();
        Self {
            description,
            complete: complete.unwrap_or(false),
            created_at,
            completed_at: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoList {
    todos: Vec<Todo>,
}

impl TodoList {
    pub fn new() -> Self {
        Self { todos: Vec::new() }
    }
}
