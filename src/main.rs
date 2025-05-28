use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

mod utils; // Import the utils module 
use utils::*;

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    text: String,
    done: bool,
}

fn get_todo_file_path() -> PathBuf {
    let mut path = dirs::home_dir().expect("Could not find home directory");
    path.push(".todo");
    fs::create_dir_all(&path).expect("Could not create directory");
    path.push("todos.json");
    path
}

fn read_todos() -> Result<Vec<Todo>, Error> {
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

fn write_todos(todos: &Vec<Todo>) -> Result<(), Error> {
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

fn print_todo_list_title() {
    println!(); // Blank line
    println!("📝 {}", "Todo List".bold());
    println!("{}", "────────────────────────────".dimmed());
}

fn list_todos() -> Result<(), Error> {
    let todos = read_todos()?;

    print_todo_list_title();

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
        let need_padding = length >= 10 && index < 10;
        let formatted_index = format_index(index, need_padding);

        let todo_row = format!("{} {} {}", formatted_index, status, todo.text);

        if todo.done {
            println!("{}", todo_row.green()); // Print done todos in green
        } else {
            println!("{}", todo_row);
        }
    }

    Ok(())
}

// TIP: This function is similar to `list_todos`, but it shows a `+` plus sign on the newly added todo (the last one)
fn list_todos_after_add() -> Result<(), Error> {
    let todos = read_todos()?;

    print_todo_list_title();

    if todos.is_empty() {
        println!("📋 Empty");
        return Ok(());
    }

    let length = todos.len();

    for (i, todo) in todos.iter().enumerate() {
        let index = i + 1;
        let status = if todo.done { "✔︎" } else { "☐" };

        // If there are more than 9 todos and the index is less than 10, we add padding
        let need_padding = length >= 10 && index < 10;
        let formatted_index = format_index(index, need_padding);

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

// This function is similar to `list_todos`, but it shows the removed todo (with a `-` minus sign instead of its number) between the previous and next todo

fn list_todos_after_remove(index: usize, removed_todo: &Todo) -> Result<(), Error> {
    let todos = read_todos()?;

    print_todo_list_title();

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
        let need_padding = length >= 10 && todo_index < 10;
        let formatted_index = format_index(todo_index, need_padding);

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

fn clear_todos() -> Result<(), Error> {
    let todos = read_todos()?;

    print_todo_list_title();

    if todos.is_empty() {
        println!("📋 Empty");
        return Ok(());
    }

    // Write an empty array to clear all todos
    write_todos(&Vec::new())?;
    println!("🗑️  All todos cleared");

    Ok(())
}

fn add_todo(text: &str) -> Result<(), Error> {
    let mut todos = read_todos()?;

    todos.push(Todo {
        text: text.to_string(),
        done: false,
    });

    write_todos(&todos)?;
    println!("Todo added: {}", text);

    // Show the updated list with the new todo
    list_todos_after_add()?;

    Ok(())
}

fn remove_todo(index: usize) -> Result<(), Error> {
    let mut todos = read_todos()?;

    // Check if the todo list is empty first
    if todos.is_empty() {
        print_todo_list_title();
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
    write_todos(&todos)?;
    println!("Todo removed: {}", todo.text);

    // Show the updated list with the removed todo
    list_todos_after_remove(index, &todo)?;

    Ok(())
}

fn toggle_done(index: usize) -> Result<(), Error> {
    let mut todos = read_todos()?;

    // Check if the todo list is empty first
    if todos.is_empty() {
        print_todo_list_title();
        println!("📋 Empty");
        return Ok(());
    }

    if index == 0 || index > todos.len() {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            format!("Invalid todo number: {}", index),
        ));
    }

    // Toggle the done status
    todos[index - 1].done = !todos[index - 1].done;
    let status = if todos[index - 1].done {
        "done"
    } else {
        "not done"
    };

    write_todos(&todos)?;
    println!("Todo marked as {}: {}", status, todos[index - 1].text);

    // Show the updated list
    list_todos()?;

    Ok(())
}

fn print_usage() {
    println!("Usage:");
    println!("  todo - List all todos");
    println!("  todo add \"Todo text\" - Add a new todo");
    println!("  todo rm - Remove the first todo");
    println!("  todo rm <number> - Remove a specific todo by number");
    println!("  todo done - Toggle the first todo completion status");
    println!("  todo done <number> - Toggle todo completion status");
    println!("  todo clear - Remove all todos");
    println!("  todo help - Show this help message");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            if let Err(e) = list_todos() {
                eprintln!("Error: {}", e);
            }
        }
        2 if args[1] == "help" => {
            print_usage();
        }
        2 if args[1] == "clear" => {
            if let Err(e) = clear_todos() {
                eprintln!("Error: {}", e);
            }
        }
        3 if args[1] == "add" => {
            if let Err(e) = add_todo(&args[2]) {
                eprintln!("Error: {}", e);
            }
        }
        2 if args[1] == "rm" => {
            // If no index is provided, remove the first todo
            if let Err(e) = remove_todo(1) {
                eprintln!("Error: {}", e);
            }
        }
        3 if args[1] == "rm" => match args[2].parse::<usize>() {
            Ok(index) => {
                if let Err(e) = remove_todo(index) {
                    eprintln!("Error: {}", e);
                }
            }
            Err(_) => {
                eprintln!("Invalid number: {}", args[2]);
            }
        },
        2 if args[1] == "done" => {
            // If no index is provided, toggle the first todo
            if let Err(e) = toggle_done(1) {
                eprintln!("Error: {}", e);
            }
        }
        3 if args[1] == "done" => match args[2].parse::<usize>() {
            Ok(index) => {
                if let Err(e) = toggle_done(index) {
                    eprintln!("Error: {}", e);
                }
            }
            Err(_) => {
                eprintln!("Invalid number: {}", args[2]);
            }
        },
        _ => {
            eprintln!("Invalid command");
            print_usage();
        }
    }
}
