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

    // Print the application introduction, quick start, help info, and usage
    print_app_intro();
    print_quick_start();
    print_help_info();
    print_usage();

    // Print each command with the correct spacing
    println!("{}", title("Commands:"));
    for cmd in &commands {
        print_command(cmd, max_length);
    }

    // TODO print aliases section in this help usage message
}

// 🔒 PRIVATE ---------------------------------

// Print the application introduction
fn print_app_intro() {
    println!("");
    println!(
        "Blazing-fast terminal {} list app {}",
        command("todo"),
        "built with Rust".italic()
    );
    println!("");
}

// Print quick start example
fn print_quick_start() {
    println!("Add your first todo with:");
    println!(
        "  {} {}",
        command("todo add"),
        "\"your todo text\"".yellow()
    );
    println!("");
}

// Print where to find more help
fn print_help_info() {
    println!("For more commands, see:");
    println!("  {} {}", command("todo"), command("help"));
    println!("");
}

// Print usage information
fn print_usage() {
    println!("{}", title("Usage:"));
    println!("  {} [COMMAND] [TEXT] [ARG]", command("todo"));
    println!("");
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
