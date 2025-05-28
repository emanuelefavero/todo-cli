use std::env;
use std::io::{Error, ErrorKind};

mod data;
mod models;
mod utils;
mod view;

fn toggle_done(index: usize) -> Result<(), Error> {
    let mut todos = data::todos::read()?;

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

    data::todos::write(&todos)?;

    // Show the updated list
    view::todos::toggled(index)?;

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
            if let Err(e) = data::todos::clear() {
                eprintln!("Error: {}", e);
            }
        }
        3 if args[1] == "add" => {
            if let Err(e) = data::todos::add(&args[2]) {
                eprintln!("Error: {}", e);
            }
        }
        2 if args[1] == "rm" => {
            // If no index is provided, remove the first todo
            if let Err(e) = data::todos::remove_todo(1) {
                eprintln!("Error: {}", e);
            }
        }
        3 if args[1] == "rm" => match args[2].parse::<usize>() {
            Ok(index) => {
                if let Err(e) = data::todos::remove_todo(index) {
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
