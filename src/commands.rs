use crate::data;
use crate::view;

// * Command line argument handler
pub fn handler(args: Vec<String>) {
    match args.len() {
        1 => {
            if let Err(e) = view::todos::all() {
                eprintln!("Error: {}", e);
            }
        }
        2 if args[1] == "help" => {
            view::help::usage();
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
        4 if args[1] == "add" => match args[3].parse::<usize>() {
            Ok(index) => {
                if let Err(e) = data::todos::add_at_index(&args[2], index) {
                    eprintln!("Error: {}", e);
                }
            }
            Err(_) => {
                eprintln!("Invalid index: {}", args[3]);
            }
        },
        2 if args[1] == "rm" => {
            // If no index is provided, remove the first todo
            if let Err(e) = data::todos::remove(1) {
                eprintln!("Error: {}", e);
            }
        }
        3 if args[1] == "rm" => match args[2].parse::<usize>() {
            Ok(index) => {
                if let Err(e) = data::todos::remove(index) {
                    eprintln!("Error: {}", e);
                }
            }
            Err(_) => {
                eprintln!("Invalid number: {}", args[2]);
            }
        },
        2 if args[1] == "done" => {
            // If no index is provided, toggle the first todo
            if let Err(e) = data::todos::toggle(1) {
                eprintln!("Error: {}", e);
            }
        }
        3 if args[1] == "done" => match args[2].parse::<usize>() {
            Ok(index) => {
                if let Err(e) = data::todos::toggle(index) {
                    eprintln!("Error: {}", e);
                }
            }
            Err(_) => {
                eprintln!("Invalid number: {}", args[2]);
            }
        },
        _ => {
            eprintln!("Invalid command");
            view::help::usage();
        }
    }
}
