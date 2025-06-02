use colored::Colorize;

use crate::data::help::{get_aliases, get_commands};
use crate::models::help::HelpCommand;
use crate::styles::help::{command, title};
use crate::utils::help::{
    calculate_components_length, calculate_max_command_length, format_command_components,
};

// 📢 PUBLIC ----------------------------------

// * Show the app usage instructions
pub fn usage() {
    print_app_intro();
    print_quick_start();
    print_usage();
    print_commands();
    print_aliases();
    print_help_info();
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
    println!("To show this help message, run:");
    println!("  {} {}", command("todo"), command("help"),);
}

// Print usage information
fn print_usage() {
    println!("{}", title("Usage:"));
    println!("  {} [COMMAND] [TEXT] [ARG]", command("todo"));
    println!("");
}

// Print commands
fn print_commands() {
    let commands = get_commands();

    // Calculate the maximum length of command combinations for proper spacing
    let max_length = calculate_max_command_length(&commands);

    println!("{}", title("Commands:"));
    for cmd in &commands {
        print_command(cmd, max_length);
    }
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

// Print aliases
fn print_aliases() {
    let aliases = get_aliases();

    // Calculate the maximum length of command combinations for proper spacing
    let max_length = calculate_max_command_length(&aliases);

    println!("{}", title("Aliases:"));
    for alias in &aliases {
        print_command(alias, max_length);
    }
    println!("");
}
