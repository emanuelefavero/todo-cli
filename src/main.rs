use std::env;

mod commands;
mod data;
mod models;
mod styles;
mod utils;
mod view;

fn main() {
    // * Collect command line arguments into a vector
    let args: Vec<String> = env::args().collect();

    // * Call the command handler
    commands::handler(args);
}
