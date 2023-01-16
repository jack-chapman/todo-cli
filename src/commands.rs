use std::fs::File;
use std::io::{Error, Write};
use std::path::Path;

use crate::todo::TodoList;

pub fn test_command(output: &str) {
    println!("TEST COMMAND: {output}");
}

pub fn init() -> Result<(), Error> {
    let todo_list = TodoList::new();

    let serialized = serde_json::to_string(&todo_list)?;

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
