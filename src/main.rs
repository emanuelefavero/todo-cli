use colored::Colorize;
use std::env;

mod commands;
mod data;
mod models;
mod styles;
mod utils;
mod view;

use data::timer;
use utils::timer::format_duration;

fn main() {
    timer::start(); // ⏱️ Start the timer

    // * Collect command line arguments into a vector
    let args: Vec<String> = env::args().collect();

    // * Call the command handler
    commands::handler(args);

    let duration = timer::stop(); // ⏱️ Check the elapsed time
    println!(
        "{}",
        format!("\n{}{}", "⚡".yellow(), format_duration(duration)).dimmed()
    );
}
