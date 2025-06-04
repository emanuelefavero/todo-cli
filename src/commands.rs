use crate::data;
use crate::errors;
use crate::view;

// * Command line argument handler
pub fn handler(args: Vec<String>) {
    match args.len() {
        1 => {
            if let Err(e) = view::todos::all() {
                errors::general(e)
            }
        }
        2 if args[1] == "help" || args[1] == "h" || args[1] == "-h" || args[1] == "--help" => {
            view::help::usage();
        }
        2 if args[1] == "clear" || args[1] == "c" => {
            if let Err(e) = data::todos::clear() {
                errors::general(e)
            }
        }
        3 if args[1] == "add" || args[1] == "a" => {
            if let Err(e) = data::todos::add(&args[2], None) {
                errors::general(e)
            }
        }
        4 if args[1] == "add" || args[1] == "a" => match args[3].parse::<usize>() {
            Ok(index) => {
                if let Err(e) = data::todos::add(&args[2], Some(index)) {
                    errors::general(e)
                }
            }
            Err(_) => eprintln!("Invalid number: {}", args[3]),
        },
        2 if args[1] == "rm" || args[1] == "r" => {
            // If no index is provided, remove the first todo
            if let Err(e) = data::todos::remove(1) {
                errors::general(e)
            }
        }
        3 if args[1] == "rm" || args[1] == "r" => match args[2].parse::<usize>() {
            Ok(index) => {
                if let Err(e) = data::todos::remove(index) {
                    errors::general(e)
                }
            }
            Err(_) => eprintln!("Invalid number: {}", args[2]),
        },
        2 if args[1] == "done" || args[1] == "d" => {
            // If no index is provided, toggle the first todo
            if let Err(e) = data::todos::toggle(1) {
                errors::general(e)
            }
        }
        3 if args[1] == "done" || args[1] == "d" => match args[2].parse::<usize>() {
            Ok(index) => {
                if let Err(e) = data::todos::toggle(index) {
                    errors::general(e)
                }
            }
            Err(_) => eprintln!("Invalid number: {}", args[2]),
        },
        4 if args[1] == "replace" || args[1] == "rp" => match args[3].parse::<usize>() {
            Ok(index) => {
                if let Err(e) = data::todos::replace(index, &args[2]) {
                    errors::general(e)
                }
            }
            Err(_) => eprintln!("Invalid number: {}", args[3]),
        },
        4 if args[1] == "insert" || args[1] == "i" => match args[3].parse::<usize>() {
            Ok(index) => {
                if let Err(e) = data::todos::add(&args[2], Some(index)) {
                    errors::general(e)
                }
            }
            Err(_) => eprintln!("Invalid number: {}", args[3]),
        },
        _ => {
            eprintln!("Invalid command");
            view::help::usage();
        }
    }
}
