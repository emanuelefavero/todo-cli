use std::fs;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

use crate::models::todo::Todo;
use crate::view;

// 🔒 PRIVATE ---------------------------------

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

// 📢 PUBLIC ----------------------------------

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

// * Clears all todos from the list
pub fn clear() -> Result<(), Error> {
    let todos = read()?;

    view::todos::title();

    if todos.is_empty() {
        println!("📋 Empty");
        return Ok(());
    }

    // Write an empty array to clear all todos
    write(&Vec::new())?;
    println!("🗑️  All todos cleared");

    Ok(())
}

// * Adds a new todo to the list
pub fn add(text: &str) -> Result<(), Error> {
    let mut todos = read()?;

    todos.push(Todo {
        text: text.to_string(),
        done: false,
    });

    write(&todos)?;

    // Show the updated list with the new todo
    view::todos::added()?;

    Ok(())
}

// * Removes a todo from the list by index
pub fn remove(index: usize) -> Result<(), Error> {
    let mut todos = read()?;

    // Check if the todo list is empty first
    if todos.is_empty() {
        view::todos::title();
        println!("📋 Empty");
        return Ok(());
    }

    if index == 0 || index > todos.len() {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            format!("Invalid todo number: {}", index),
        ));
    }

    let todo = todos.remove(index - 1);
    write(&todos)?;

    // Show the updated list with the removed todo
    view::todos::removed(index, &todo)?;

    Ok(())
}
