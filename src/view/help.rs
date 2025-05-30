use colored::Colorize;

use crate::data::help::get_commands;
use crate::models::help::HelpCommand;
use crate::styles::help::{command, title};
use crate::utils::help::{
    calculate_components_length, calculate_max_command_length, format_command_components,
};

// 📢 PUBLIC ----------------------------------

// * Show the app usage instructions
pub fn usage() {
    // Get all help commands
    let commands = get_commands();

    // Calculate the maximum length of command combinations for proper spacing
    let max_length = calculate_max_command_length(&commands);

    // Print the header
    print_header();

    // Print each command with the correct spacing
    for cmd in &commands {
        print_command(cmd, max_length);
    }
}

// 🔒 PRIVATE ---------------------------------

// Print the help header
fn print_header() {
    println!("");
    println!(
        "Snappy and minimal {} list app for your terminal {}",
        command("todo"),
        "built with Rust".italic()
    );
    println!("");
    println!("{}", title("Usage:"));
    println!("");
    println!("{}", title("Commands:"));
}

// Print a single command with proper formatting
fn print_command(cmd: &HelpCommand, max_length: usize) {
    let (cmd_formatted, cmd_text, cmd_arg) = format_command_components(cmd);
    let components_length = calculate_components_length(cmd);
    let spaces = " ".repeat(max_length - components_length + 1);

    println!(
        "  {}{}{}{} {}",
        cmd_formatted, cmd_text, cmd_arg, spaces, cmd.description
    );
}
