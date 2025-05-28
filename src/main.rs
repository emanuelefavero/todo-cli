use std::env;
use std::io::{Error, ErrorKind};

mod data;
use data::todos::{read_todos, write_todos};
mod models;
use models::todo::Todo;
mod utils;
mod view;

fn clear_todos() -> Result<(), Error> {
    let todos = read_todos()?;

    view::todos::title();

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

    // Show the updated list with the new todo
    view::todos::list_todos_after_add()?;

    Ok(())
}

fn remove_todo(index: usize) -> Result<(), Error> {
    let mut todos = read_todos()?;

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
    write_todos(&todos)?;

    // Show the updated list with the removed todo
    view::todos::list_todos_after_remove(index, &todo)?;

    Ok(())
}

fn toggle_done(index: usize) -> Result<(), Error> {
    let mut todos = read_todos()?;

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

    // Toggle the done status
    todos[index - 1].done = !todos[index - 1].done;

    write_todos(&todos)?;

    // Show the updated list
    view::todos::list_todos_after_toggle(index)?;

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
            if let Err(e) = view::todos::all() {
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
