use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    text: String,
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
        Error::new(ErrorKind::InvalidData, format!("Could not parse todos: {}", e))
    })?;
    
    Ok(todos)
}

fn write_todos(todos: &Vec<Todo>) -> Result<(), Error> {
    let path = get_todo_file_path();
    let content = serde_json::to_string_pretty(todos).map_err(|e| {
        Error::new(ErrorKind::InvalidData, format!("Could not serialize todos: {}", e))
    })?;
    
    fs::write(path, content)?;
    Ok(())
}

fn list_todos() -> Result<(), Error> {
    let todos = read_todos()?;
    
    if todos.is_empty() {
        println!("No todos found");
        return Ok(());
    }
    
    println!("📝 Todo List");
    println!("────────────────────────────");
    
    for (i, todo) in todos.iter().enumerate() {
        println!("{} {}", i + 1, todo.text);
    }
    
    Ok(())
}

fn add_todo(text: &str) -> Result<(), Error> {
    let mut todos = read_todos()?;
    
    todos.insert(0, Todo {
        text: text.to_string(),
    });
    
    write_todos(&todos)?;
    println!("Todo added: {}", text);
    
    Ok(())
}

fn remove_todo(index: usize) -> Result<(), Error> {
    let mut todos = read_todos()?;
    
    if index == 0 || index > todos.len() {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            format!("Invalid todo number: {}", index),
        ));
    }
    
    let todo = todos.remove(index - 1);
    write_todos(&todos)?;
    println!("Todo removed: {}", todo.text);
    
    Ok(())
}

fn print_usage() {
    println!("Usage:");
    println!("  todo - List all todos");
    println!("  todo add \"Todo text\" - Add a new todo");
    println!("  todo rm <number> - Remove a todo by number");
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
        3 if args[1] == "add" => {
            if let Err(e) = add_todo(&args[2]) {
                eprintln!("Error: {}", e);
            }
        }
        3 if args[1] == "rm" => {
            match args[2].parse::<usize>() {
                Ok(index) => {
                    if let Err(e) = remove_todo(index) {
                        eprintln!("Error: {}", e);
                    }
                }
                Err(_) => {
                    eprintln!("Invalid number: {}", args[2]);
                }
            }
        }
        _ => {
            eprintln!("Invalid command");
            print_usage();
        }
    }
}
