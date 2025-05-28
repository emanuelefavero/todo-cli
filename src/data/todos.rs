use std::fs;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

use crate::models::todo::Todo;

// ? Utility function to get the path to the todo file
fn get_todo_file_path() -> PathBuf {
    let mut path = dirs::home_dir().expect("Could not find home directory");
    path.push(".todo");
    fs::create_dir_all(&path).expect("Could not create directory");
    path.push("todos.json");
    path
}

// * Reads todos from a JSON file
pub fn read() -> Result<Vec<Todo>, Error> {
    let path = get_todo_file_path();

    if !path.exists() {
        fs::write(&path, "[]").expect("Could not create todo file");
        return Ok(Vec::new());
    }

    let content = fs::read_to_string(&path)?;
    let todos: Vec<Todo> = serde_json::from_str(&content).map_err(|e| {
        Error::new(
            ErrorKind::InvalidData,
            format!("Could not parse todos: {}", e),
        )
    })?;

    Ok(todos)
}

// * Writes todos to a JSON file
pub fn write_todos(todos: &Vec<Todo>) -> Result<(), Error> {
    let path = get_todo_file_path();
    let content = serde_json::to_string_pretty(todos).map_err(|e| {
        Error::new(
            ErrorKind::InvalidData,
            format!("Could not serialize todos: {}", e),
        )
    })?;

    fs::write(path, content)?;
    Ok(())
}
