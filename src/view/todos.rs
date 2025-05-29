use std::io::Error;

use colored::Colorize;

use crate::data;
use crate::models::todo::Todo;
use crate::utils;

// * Show the title of the todo list
pub fn title() {
    println!(); // Blank line
    println!("📝 {}", "Todo List".bold());
    println!("{}", "────────────────────────────".dimmed());
}

// * Show all todos in the list
pub fn all() -> Result<(), Error> {
    let todos = data::todos::read()?;

    title(); // Show the title

    // If the todo list is empty, show Empty message
    if todos.is_empty() {
        println!("📋 Empty");
        return Ok(());
    }

    // Check if we have 10 or more todos to determine padding of first 9 todos
    let length = todos.len();

    for (i, todo) in todos.iter().enumerate() {
        // Index
        let index = i + 1;
        let formatted_index = utils::format::pad_index(index, length); // Add padding to index less than 10

        // Status
        let status = if todo.done { "✔︎" } else { "☐" };
        let formatted_status = if todo.done {
            status.green()
        } else {
            status.blue()
        };

        // Print the todo
        println!(
            "{} {} {}",
            formatted_index.purple(),
            formatted_status,
            todo.text
        );
    }

    Ok(())
}

// * Show the newly added todo after adding it to the list
pub fn added() -> Result<(), Error> {
    let todos = data::todos::read()?;

    title(); // Show the title

    // If the todo list is empty, show Empty message
    if todos.is_empty() {
        println!("📋 Empty");
        return Ok(());
    }

    // Check if we have 10 or more todos to determine padding of first 9 todos
    let length = todos.len();

    // Show the todos
    for (i, todo) in todos.iter().enumerate() {
        // Index
        let index = i + 1;
        let formatted_index = utils::format::pad_index(index, length); // Add padding to index less than 10

        // Status
        let status = if todo.done { "✔︎" } else { "☐" };
        let formatted_status = if todo.done {
            status.green()
        } else {
            status.blue()
        };

        // Print the added todo (last one in the list)
        if i == todos.len() - 1 {
            let todo_row = format!("{} + {}", formatted_index, todo.text);
            println!("{}", todo_row.cyan());

        // Print the rest of the todos
        } else {
            println!(
                "{} {} {}",
                formatted_index.purple(),
                formatted_status,
                todo.text
            );
        }
    }

    Ok(())
}

// * Show the removed todo after removing it from the list
pub fn removed(index: usize, removed_todo: &Todo) -> Result<(), Error> {
    let todos = data::todos::read()?;

    title(); // Show the title

    // If the todo list is empty, show Empty message
    if todos.is_empty() {
        // Add padding to the removed todo row if needed
        let removed_todo_row = format!("-  {}", removed_todo.text.strikethrough());
        println!("{}", removed_todo_row.red()); // show the removed todo
        println!("📋 Empty");
        return Ok(());
    }

    // Check if we have 10 or more todos to determine padding of first 9 todos
    let length = todos.len();

    // Show the todos
    for (i, todo) in todos.iter().enumerate() {
        let todo_index = i + 1;
        let formatted_index = utils::format::pad_index(todo_index, length); // Add padding to index less than 10

        let status = if todo.done { "✔︎" } else { "☐" };
        let formatted_status = if todo.done {
            status.green()
        } else {
            status.blue()
        };

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

        // Print the current todo
        println!(
            "{} {} {}",
            formatted_index.purple(),
            formatted_status,
            todo.text
        );
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

// * Show the toggled todo (done/undone) after toggling its status
pub fn toggled(index: usize) -> Result<(), Error> {
    let todos = data::todos::read()?;

    title(); // Show the title

    // If the todo list is empty, show Empty message
    if todos.is_empty() {
        println!("📋 Empty");
        return Ok(());
    }

    // Check if we have 10 or more todos to determine padding of first 9 todos
    let length = todos.len();

    // Show the todos
    for (i, todo) in todos.iter().enumerate() {
        // Index
        let todo_index = i + 1;
        let formatted_index = utils::format::pad_index(todo_index, length); // Add padding to index less than 10

        // Status
        let status = if todo.done { "✔︎" } else { "☐" };
        let formatted_status = if todo.done {
            status.green()
        } else {
            status.blue()
        };

        // Format the todo row
        let todo_row = format!(
            "{} {} {}",
            formatted_index.purple(),
            formatted_status,
            todo.text
        );
        let toggled_todo_row = format!("{} {} {}", formatted_index, formatted_status, todo.text);
        let marker = "✦".yellow();

        // If the current todo is the one that was toggled, highlight it
        if todo_index == index {
            if todo.done {
                println!("{} {}", toggled_todo_row.green(), marker); // Done in green
            } else {
                println!("{} {}", toggled_todo_row.blue(), marker); // Undone with marker
            }
        // Otherwise, print the todo normally
        } else {
            println!("{}", todo_row);
        }
    }

    Ok(())
}
