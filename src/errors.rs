use crate::models::todo::Todo;
use colored::Colorize;
use std::io::{Error, ErrorKind};

pub fn general(error: Error) {
    eprintln!("{} \n{}", "Error:".dimmed(), error);
}

pub fn invalid_number(index: &str) {
    eprintln!("{} {}", "Invalid number:".red(), index.yellow())
}

pub fn invalid_number_with_length(index: usize, todos: &[Todo]) -> Error {
    Error::new(
        ErrorKind::InvalidInput,
        format!(
            "{} {} \nThe todo list has {} items \n{}",
            "Invalid number:".red(),
            index.to_string().yellow(),
            todos.len().to_string().cyan(),
            format!("Run {} to see the list", "`todo`".bold().green()).dimmed()
        ),
    )
}

pub fn invalid_number_zero() -> Error {
    Error::new(
        ErrorKind::InvalidInput,
        format!(
            "{} {} \nUse 1 for the first todo \n{}",
            "Invalid number:".red(),
            "0".yellow(),
            format!("Run {} to see the list", "`todo`".bold().green()).dimmed()
        ),
    )
}
