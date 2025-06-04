use std::io::Error;

pub fn general(error: Error) {
    eprintln!("Error: {}", error);
}
