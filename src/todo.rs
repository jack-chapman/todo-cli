use std::{
    collections::BTreeMap,
    fs::File,
    io::{Error, Read, Write},
    path::Path,
};

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub description: String,
    pub complete: bool,
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
    pub todos: BTreeMap<usize, Todo>,
    current_index: usize,
}

impl TodoList {
    pub fn new() -> Self {
        Self {
            todos: BTreeMap::new(),
            current_index: 0,
        }
    }
    pub fn from_file(path: &str) -> Result<Self, String> {
        let path = Path::new(path);

        if !path.exists() {
            return Err(String::from("todo file not found"));
        }

        if let Ok(mut file) = File::open(path) {
            let mut buffer = String::new();
            if let Err(e) = file.read_to_string(&mut buffer) {
                return Err(e.to_string());
            }

            if let Ok(todo) = serde_json::from_str::<Self>(buffer.as_str()) {
                Ok(todo)
            } else {
                Err(String::from("cannot load todo file"))
            }
        } else {
            Err(String::from("cannot open todo file"))
        }
    }

    pub fn to_new_file(&self) -> Result<(), Error> {
        let serialized = serde_json::to_string(self)?;

        let path = Path::new("todo_list.json");

        if path.exists() {
            return Err(Error::new(
                std::io::ErrorKind::AlreadyExists,
                "todo_list.json already exists in cwd.",
            ));
        }

        let mut output = File::create(path)?;

        write!(output, "{serialized}")?;

        Ok(())
    }

    pub fn to_file(&self) -> Result<(), Error> {
        let serialized = serde_json::to_string(self)?;

        let path = Path::new("todo_list.json");

        let mut output = File::create(path)?;

        write!(output, "{serialized}")?;

        Ok(())
    }

    fn next_index(&mut self) -> usize {
        self.current_index += 1;
        self.current_index
    }

    pub fn add(&mut self, todo: Todo) -> Option<Todo> {
        let index = self.next_index();
        self.todos.insert(index, todo)
    }
}
