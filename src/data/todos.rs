use std::fs;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

use crate::models::todo::Todo;
use crate::view;

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

// // * Count the number of todos in the list
// pub fn count() -> Result<usize, Error> {
//     let todos = read()?;
//     Ok(todos.len())
// }

// * Clears all todos from the list
pub fn clear() -> Result<(), Error> {
    let todos = read()?;

    view::todos::title();

    if todos.is_empty() {
        view::todos::empty();
        return Ok(());
    }

    // Write an empty array to clear all todos
    write(&Vec::new())?;
    println!("🗑️  All todos cleared");

    Ok(())
}

// * Adds a new todo to the list, optionally at a specific index
pub fn add(text: &str, index: Option<usize>) -> Result<(), Error> {
    let mut todos = read()?;

    // Create the new todo
    let new_todo = Todo {
        text: text.to_string(),
        done: false,
    };

    // Either insert at a specific index or add to the end
    // TIP: `match index` is used to handle both cases
    match index {
        // If an index is provided, insert at that position
        Some(idx) => {
            // Check if the index is valid
            if idx == 0 || idx > todos.len() + 1 {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!(
                        "Invalid number: {}. The todo list has {} items.",
                        idx,
                        todos.len()
                    ),
                ));
            }

            // Insert the new todo at the specified index
            todos.insert(idx - 1, new_todo);
        }
        // If no index is provided, add to the end of the list
        None => {
            // Add to the end of the list (default behavior)
            todos.push(new_todo);
        }
    }

    write(&todos)?;

    // Show the updated list with the new todo highlighted
    view::todos::added(index)?;

    Ok(())
}

// * Removes a todo from the list by index
pub fn remove(index: usize) -> Result<(), Error> {
    let mut todos = read()?;

    // Check if the todo list is empty first
    if todos.is_empty() {
        view::todos::title();
        view::todos::empty();
        return Ok(());
    }

    if index == 0 || index > todos.len() {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            format!(
                "Invalid number: {}. The todo list has {} items.",
                index,
                todos.len()
            ),
        ));
    }

    let todo = todos.remove(index - 1);
    write(&todos)?;

    // Show the updated list with the removed todo
    view::todos::removed(index, &todo)?;

    Ok(())
}

// * Toggles the done status of a todo by index
pub fn toggle(index: usize) -> Result<(), Error> {
    let mut todos = read()?;

    // Check if the todo list is empty first
    if todos.is_empty() {
        view::todos::title();
        view::todos::empty();
        return Ok(());
    }

    if index == 0 || index > todos.len() {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            format!(
                "Invalid number: {}. The todo list has {} items.",
                index,
                todos.len()
            ),
        ));
    }

    // Toggle the done status
    todos[index - 1].done = !todos[index - 1].done;

    write(&todos)?;

    // Show the updated list
    view::todos::toggled(index)?;

    Ok(())
}

// * Replaces the text of a todo at a specific index
pub fn replace(index: usize, new_text: &str) -> Result<(), Error> {
    let mut todos = read()?;

    // Check if the todo list is empty first
    if todos.is_empty() {
        view::todos::title();
        view::todos::empty();
        return Ok(());
    }

    if index == 0 || index > todos.len() {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            format!(
                "Invalid number: {}. The todo list has {} items.",
                index,
                todos.len()
            ),
        ));
    }

    // Save the old todo text before replacing
    let old_text = todos[index - 1].text.clone();

    // Replace the todo at the specified index
    todos[index - 1].text = new_text.to_string();

    write(&todos)?;

    // Show the updated list with the replaced todo, passing both old and new text
    view::todos::replaced(index, &old_text, new_text)?;

    Ok(())
}

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
