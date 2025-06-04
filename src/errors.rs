use crate::models::todo::Todo;
use colored::Colorize;
use std::io::{Error, ErrorKind};

pub fn general(error: Error) {
    eprintln!("{} {}", "Error:".red(), error);
}

pub fn invalid_number_with_length(index: usize, todos: &[Todo]) -> Error {
    Error::new(
        ErrorKind::InvalidInput,
        format!(
            "Invalid number: {}. The todo list has {} items.",
            index,
            todos.len()
        ),
    )
}
