use std::fs;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

use crate::models::todo::Todo;

// ? Creates the file path for the todo file
fn file_path() -> PathBuf {
    // Get the home directory
    let mut path = dirs::home_dir().expect("Could not find home directory");

    // NOTE: Choose the directory where the todo file will be stored
    path.push(".todo");

    // Create the directory if it doesn't exist
    fs::create_dir_all(&path).expect("Could not create directory: .todo/");

    path.push("todos.json"); // Append the filename to the path
    path // Return the full path to the todo file
}

// * Reads todos from a JSON file
pub fn read() -> Result<Vec<Todo>, Error> {
    let path = file_path();

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
pub fn write(todos: &Vec<Todo>) -> Result<(), Error> {
    let path = file_path();
    let content = serde_json::to_string_pretty(todos).map_err(|e| {
        Error::new(
            ErrorKind::InvalidData,
            format!("Could not serialize todos: {}", e),
        )
    })?;

    fs::write(path, content)?;
    Ok(())
}
