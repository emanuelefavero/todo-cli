use std::io::Error;

use crate::errors;
use crate::models::todo::Todo;

// * Returns a formatted index string with left padding if needed
pub fn pad_index(index: usize, length: usize) -> String {
    let need_padding = length >= 10 && index < 10;

    if need_padding {
        format!(" {}", index) // Index with left padding
    } else {
        index.to_string() // No padding needed
    }
}

// * Checks if the index is between 1 and the length of the todo list
pub fn validate_index(index: usize, todos: &[Todo]) -> Result<(), Error> {
    if index == 0 {
        return Err(errors::invalid_number_zero());
    }

    if index > todos.len() {
        return Err(errors::invalid_number_with_length(index, todos));
    }

    Ok(())
}

// * Checks if the index is between 1 and the length of the todo list + 1
// ? This allows adding a new todo at the end of the list
pub fn validate_index_on_add(index: usize, todos: &[Todo]) -> Result<(), Error> {
    if index == 0 {
        return Err(errors::invalid_number_zero());
    }

    if index > todos.len() + 1 {
        return Err(errors::invalid_number_with_length(index, todos));
    }

    Ok(())
}
