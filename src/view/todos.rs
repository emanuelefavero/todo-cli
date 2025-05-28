use std::io::Error;

use colored::Colorize;

use crate::data::todos::read_todos;
use crate::utils::format::format_index;

pub fn title() {
    println!(); // Blank line
    println!("📝 {}", "Todo List".bold());
    println!("{}", "────────────────────────────".dimmed());
}

pub fn list_todos() -> Result<(), Error> {
    let todos = read_todos()?;

    title();

    if todos.is_empty() {
        println!("📋 Empty");
        return Ok(());
    }

    // Check if we have 10 or more todos to determine padding of first 9 todos
    let length = todos.len();

    for (i, todo) in todos.iter().enumerate() {
        let index = i + 1;
        let status = if todo.done { "✔︎" } else { "☐" };

        // If there are more than 9 todos and the index is less than 10, we add padding
        let formatted_index = format_index(index, length);

        let todo_row = format!("{} {} {}", formatted_index, status, todo.text);

        if todo.done {
            println!("{}", todo_row.green()); // Print done todos in green
        } else {
            println!("{}", todo_row);
        }
    }

    Ok(())
}
