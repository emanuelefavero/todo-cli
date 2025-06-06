use crate::data;
use crate::errors;
use crate::view;

// TODO implement "clear screen" functionality in the `setup_todos_view` function and all other commands (e.g. clear command) with `crossterm` library

// * Command line argument handler
pub fn handler(args: Vec<String>) {
    match args.len() {
        // * no arguments - displays the todo list (e.g. `todo`)
        1 => {
            if let Err(e) = view::todos::all() {
                errors::general(e)
            }
        }

        // * help - displays the usage (e.g. `todo help`)
        2 if args[1] == "help" || args[1] == "h" || args[1] == "-h" || args[1] == "--help" => {
            view::help::usage();
        }

        // * clear - clears the todo list (e.g. `todo clear`)
        2 if args[1] == "clear" || args[1] == "c" => {
            if let Err(e) = data::todos::clear() {
                errors::general(e)
            }
        }

        // * add <TEXT> - adds a new todo (e.g. `todo add "Buy milk"`)
        3 if args[1] == "add" || args[1] == "a" => {
            if let Err(e) = data::todos::add(&args[2], None) {
                errors::general(e)
            }
        }

        // * add <TEXT> <INDEX> - adds todo at index
        // ? (e.g. `todo add "Buy milk" 2`)
        4 if args[1] == "add" || args[1] == "a" => match args[3].parse::<usize>() {
            Ok(index) => {
                if let Err(e) = data::todos::add(&args[2], Some(index)) {
                    errors::general(e)
                }
            }
            Err(_) => errors::invalid_number(&args[3]),
        },

        // * rm - removes first todo (e.g. `todo rm`)
        2 if args[1] == "rm" || args[1] == "remove" || args[1] == "r" => {
            // If no index is provided, remove the first todo
            if let Err(e) = data::todos::remove(1) {
                errors::general(e)
            }
        }

        // * rm <INDEX> - removes todo at index (e.g. `todo rm 2`)
        3 if args[1] == "rm" || args[1] == "r" => match args[2].parse::<usize>() {
            Ok(index) => {
                if let Err(e) = data::todos::remove(index) {
                    errors::general(e)
                }
            }
            Err(_) => errors::invalid_number(&args[2]),
        },

        // * done - toggles first todo (e.g. `todo done`)
        2 if args[1] == "done" || args[1] == "d" => {
            if let Err(e) = data::todos::toggle(1) {
                errors::general(e)
            }
        }

        // * done <INDEX> - toggles todo at index (e.g. `todo done 2`)
        3 if args[1] == "done" || args[1] == "d" => match args[2].parse::<usize>() {
            Ok(index) => {
                if let Err(e) = data::todos::toggle(index) {
                    errors::general(e)
                }
            }
            Err(_) => errors::invalid_number(&args[2]),
        },

        // * replace <TEXT> <INDEX> - replaces a todo at index
        // ? e.g. `todo replace "Buy milk" 2`
        4 if args[1] == "replace" || args[1] == "rp" => match args[3].parse::<usize>() {
            Ok(index) => {
                if let Err(e) = data::todos::replace(index, &args[2]) {
                    errors::general(e)
                }
            }
            Err(_) => errors::invalid_number(&args[3]),
        },

        // * insert <TEXT> <INDEX> - inserts a todo at index
        // ? e.g. `todo insert "Buy milk" 2`
        4 if args[1] == "insert" || args[1] == "i" => match args[3].parse::<usize>() {
            Ok(index) => {
                if let Err(e) = data::todos::add(&args[2], Some(index)) {
                    errors::general(e)
                }
            }
            Err(_) => errors::invalid_number(&args[3]),
        },

        // * - any other command - displays the usage (e.g. `todo unknown`)
        _ => {
            eprintln!("Invalid command");
            view::help::usage();
        }
    }
}
