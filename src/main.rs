use std::env;

mod commands;
mod data;
mod models;
mod styles;
mod utils;
mod view;

use data::timer;

fn main() {
    timer::start(); // ⏱️ Start the timer

    // * Collect command line arguments into a vector
    let args: Vec<String> = env::args().collect();

    // * Call the command handler
    commands::handler(args);
}
