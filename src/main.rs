use colored::Colorize;
use std::env;
use std::time::{Duration, Instant}; // ? For measuring execution time

mod commands;
mod data;
mod models;
mod styles;
mod utils;
mod view;

// TODO put `format_duration` in a utils/time.rs module

fn format_duration(duration: Duration) -> String {
    if duration.as_millis() > 0 {
        format!("{}ms", duration.as_millis())
    } else if duration.as_micros() > 0 {
        format!("{}µs", duration.as_micros())
    } else {
        format!("{}ns", duration.as_nanos())
    }
}

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
