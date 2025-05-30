use std::io::Error;

use colored::{ColoredString, Colorize};

use crate::data;
use crate::models::todo::Todo;
use crate::utils;

// 📢 PUBLIC ----------------------------------

// * Show the title of the todo list
pub fn title() {
    println!(); // Blank line
    println!("📝 {}", "Todo List".bold());
    println!("{}", "────────────────────────────".dimmed());
}

// * Show Empty message
pub fn empty() {
    println!("{}", "📋 Empty".dimmed());
}

// * Show all todos in the list
pub fn all() -> Result<(), Error> {
    let todos = setup_todos_view()?;

    if todos.is_empty() {
        return Ok(());
    }

    let length = todos.len();

    for (i, todo) in todos.iter().enumerate() {
        let index = i + 1;
        let (formatted_index, formatted_status) = format_todo(index, todo, length);
        print_todo(&formatted_index, &formatted_status, &todo.text);
    }

    Ok(())
}

// * Show the newly added todo after adding it to the list
pub fn added(index: Option<usize>) -> Result<(), Error> {
    let todos = setup_todos_view()?;

    if todos.is_empty() {
        return Ok(());
    }

    let length = todos.len();

    // Show the todos
    for (i, todo) in todos.iter().enumerate() {
        let todo_index = i + 1;
        let (formatted_index, formatted_status) = format_todo(todo_index, todo, length);

        // Check if the current todo is the newly added one
        // TIP: `match index` is needed to determine which command was used to add the todo
        let is_newly_added_todo = match index {
            Some(idx) => todo_index == idx, // Return true if the provided index is equal to the current index

            // TIP: If an index is provided, it means that the command was `todo add "Todo text" <number>`, so we check if the current index matches the provided index
            None => i == todos.len() - 1, // Return true if no index is provided and the current index is the last one

                                          // TIP: If no index is provided, it means that the command was `todo add "Todo text"` without a specific index, so the newly added todo is the last one in the list
        };

        // If the current todo is the newly added one, highlight it
        if is_newly_added_todo {
            let todo_row = format!("{} + {}", formatted_index, todo.text);
            println!("{}", todo_row.cyan());

        // Otherwise, print the todo normally
        } else {
            // Print regular todos
            print_todo(&formatted_index, &formatted_status, &todo.text);
        }
    }

    Ok(())
}

// * Show the removed todo after removing it from the list
pub fn removed(index: usize, removed_todo: &Todo) -> Result<(), Error> {
    let todos = setup_todos_view()?;

    if todos.is_empty() {
        // Add padding to the removed todo row if needed
        let removed_todo_row = format!("-  {}", removed_todo.text.strikethrough());
        println!("{}", removed_todo_row.red()); // show the removed todo
        return Ok(());
    }

    let length = todos.len();

    // Show the todos
    for (i, todo) in todos.iter().enumerate() {
        let todo_index = i + 1;
        let (formatted_index, formatted_status) = format_todo(todo_index, todo, length);

        // Show removed todo after the todo at its previous position
        if i == index - 1 {
            let removed_todo_row = format_removed_todo(removed_todo, length);
            println!("{}", removed_todo_row.red()); // show the removed todo
        }

        // Print the current todo
        print_todo(&formatted_index, &formatted_status, &todo.text);
    }

    // If the removed todo was the last one, show it at the end
    if index == todos.len() + 1 {
        let removed_todo_row = format_removed_todo(removed_todo, length);
        println!("{}", removed_todo_row.red()); // show the removed todo at the end
    }

    Ok(())
}

// * Show the toggled todo (done/undone) after toggling its status
pub fn toggled(index: usize) -> Result<(), Error> {
    let todos = setup_todos_view()?;

    if todos.is_empty() {
        return Ok(());
    }

    let length = todos.len();

    // Show the todos
    for (i, todo) in todos.iter().enumerate() {
        let todo_index = i + 1;
        let (formatted_index, formatted_status) = format_todo(todo_index, todo, length);

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

// 🔒 PRIVATE ---------------------------------

// ? Helper function to setup todos view and handle empty list case
fn setup_todos_view() -> Result<Vec<Todo>, Error> {
    let todos = data::todos::read()?;

    title(); // Show the title

    if todos.is_empty() {
        empty(); // Show empty message
    }

    Ok(todos)
}

// ? Helper function to format a todo's index and status
fn format_todo(index: usize, todo: &Todo, list_length: usize) -> (String, ColoredString) {
    // Format index with padding if needed
    let formatted_index = utils::todos::pad_index(index, list_length);

    // Format status with color
    let status = if todo.done { "✔︎" } else { "☐" };
    let formatted_status = if todo.done {
        status.green()
    } else {
        status.blue()
    };

    (formatted_index, formatted_status)
}

// ? Helper function to print a standard todo item
fn print_todo(index_str: &str, status: &ColoredString, text: &str) {
    println!("{} {} {}", index_str.purple(), status, text);
}

// ? Helper function to format a removed todo
fn format_removed_todo(todo: &Todo, list_length: usize) -> String {
    let removed_status = if todo.done { "✔︎" } else { "☐" };
    let padding = if list_length >= 10 { " " } else { "" };

    format!(
        "{}- {} {}",
        padding,
        removed_status,
        todo.text.strikethrough()
    )
}
