use std::io::Error;

use colored::Colorize;

use crate::data::todos::read_todos;
use crate::models::todo::Todo;
use crate::utils::format::format_index;

pub fn title() {
    println!(); // Blank line
    println!("📝 {}", "Todo List".bold());
    println!("{}", "────────────────────────────".dimmed());
}

pub fn all() -> Result<(), Error> {
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

// TIP: This function is similar to `all`, but it shows a `+` plus sign on the newly added todo (the last one)
pub fn list_todos_after_add() -> Result<(), Error> {
    let todos = read_todos()?;

    title();

    if todos.is_empty() {
        println!("📋 Empty");
        return Ok(());
    }

    let length = todos.len();

    for (i, todo) in todos.iter().enumerate() {
        let index = i + 1;
        let status = if todo.done { "✔︎" } else { "☐" };

        // If there are more than 9 todos and the index is less than 10, we add padding
        let formatted_index = format_index(index, length);

        if i == todos.len() - 1 {
            let todo_row = format!("{} + {}", formatted_index, todo.text);
            println!("{}", todo_row.blue()); // print last todo
        } else if todo.done {
            println!(
                "{}",
                format!("{} {} {}", formatted_index, status, todo.text).green()
            );
        } else {
            println!("{} {} {}", formatted_index, status, todo.text);
        }
    }

    Ok(())
}

// This function is similar to `all`, but it shows the removed todo (with a `-` minus sign instead of its number) between the previous and next todo

pub fn list_todos_after_remove(index: usize, removed_todo: &Todo) -> Result<(), Error> {
    let todos = read_todos()?;

    title();

    if todos.is_empty() {
        // Add padding to the removed todo row if needed
        let removed_todo_row = format!("-  {}", removed_todo.text.strikethrough());
        println!("{}", removed_todo_row.red()); // show the removed todo
        println!("📋 Empty");
        return Ok(());
    }

    // Check if we have 10 or more todos to determine padding of first 9 todos
    let length = todos.len();

    for (i, todo) in todos.iter().enumerate() {
        let todo_index = i + 1;
        let status = if todo.done { "✔︎" } else { "☐" };

        // If there are more than 9 todos and the index is less than 10, we add padding
        let formatted_index = format_index(todo_index, length);

        // Show removed todo after the todo at its previous position
        if i == index - 1 {
            let removed_status = if removed_todo.done { "✔︎" } else { "☐" };
            // Add padding to the removed todo row if there are 10 or more todos
            let removed_todo_padding = if length >= 10 { " " } else { "" };
            let removed_todo_row = format!(
                "{}- {} {}",
                removed_todo_padding,
                removed_status,
                removed_todo.text.strikethrough()
            );
            println!("{}", removed_todo_row.red()); // show the removed todo
        }

        if todo.done {
            println!(
                "{}",
                format!("{} {} {}", formatted_index, status, todo.text).green()
            );
        } else {
            println!("{} {} {}", formatted_index, status, todo.text);
        }
    }

    // If the removed todo was the last one, show it at the end
    if index == todos.len() + 1 {
        let removed_status = if removed_todo.done { "✔︎" } else { "☐" };
        // Add padding to the removed todo row if there are 10 or more todos
        let removed_todo_padding = if length >= 10 { " " } else { "" };
        let removed_todo_row = format!(
            "{}- {} {}",
            removed_todo_padding,
            removed_status,
            removed_todo.text.strikethrough()
        );
        println!("{}", removed_todo_row.red()); // show the removed todo at the end
    }

    Ok(())
}

// This function is similar to `all`, but it shows the toggled todo with a special message after the todo text so users can directly see which todo was toggled
pub fn list_todos_after_toggle(index: usize) -> Result<(), Error> {
    let todos = read_todos()?;

    title();

    if todos.is_empty() {
        println!("📋 Empty");
        return Ok(());
    }

    // Check if we have 10 or more todos to determine padding of first 9 todos
    let length = todos.len();

    for (i, todo) in todos.iter().enumerate() {
        let todo_index = i + 1;
        let status = if todo.done { "✔︎" } else { "☐" };

        // If there are more than 9 todos and the index is less than 10, we add padding
        let formatted_index = format_index(todo_index, length);

        let todo_row = format!("{} {} {}", formatted_index, status, todo.text);

        if todo_index == index {
            // If this is the toggled todo, show it with a special message
            if todo.done {
                let message = "✦".yellow();
                println!("{} {}", todo_row.green(), message);
            } else {
                let message = "✦".yellow();
                println!("{} {}", todo_row, message);
            }
        } else if todo.done {
            println!("{}", todo_row.green()); // Print done todos in green
        } else {
            println!("{}", todo_row);
        }
    }

    Ok(())
}
