use colored::Colorize;
use std::env;
use std::time::Instant; // ? For measuring execution time

mod commands;
mod data;
mod models;
mod styles;
mod utils;
mod view;

use utils::time::format_duration;

fn main() {
    let start = Instant::now(); // ⏱️ Start the timer

    // * Collect command line arguments into a vector
    let args: Vec<String> = env::args().collect();

    // * Call the command handler
    commands::handler(args);

    let duration = start.elapsed(); // ⏱️ Check the elapsed time
    println!(
        "{}",
        format!("\n{}{}", "⚡".yellow(), format_duration(duration)).dimmed()
    );
}
