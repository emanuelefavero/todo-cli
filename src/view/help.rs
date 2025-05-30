use crate::data::help::get_commands;
use crate::models::help::HelpCommand;
use colored::Colorize;

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

// Iterate through commands and calculate the maximum length of all command combinations (command + text + arg)
fn calculate_max_command_length(commands: &[HelpCommand]) -> usize {
    commands
        .iter()
        .map(|cmd| {
            cmd.command.len()
                + cmd.command_text.as_ref().map_or(0, |t| t.len() + 1)
                + cmd.command_arg.as_ref().map_or(0, |a| a.len() + 1)
        })
        .max() // Find the maximum length
        .unwrap_or(0) // Default to 0 if no commands are present
}

// Format a command's components (command, text, arg)
fn format_command_components(cmd: &HelpCommand) -> (colored::ColoredString, String, String) {
    let cmd_text = cmd
        .command_text
        .as_ref()
        .map_or(String::new(), |text| format!(" {}", command_text(text)));

    let cmd_arg = cmd
        .command_arg
        .as_ref()
        .map_or(String::new(), |arg| format!(" {}", command_arg(arg)));

    (command(&cmd.command), cmd_text, cmd_arg)
}

// Calculate the length of a command's components for spacing
fn calculate_components_length(cmd: &HelpCommand) -> usize {
    cmd.command.len()
        + cmd.command_text.as_ref().map_or(0, |t| t.len() + 1)
        + cmd.command_arg.as_ref().map_or(0, |a| a.len() + 1)
}

// Print the help header
fn print_header() {
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

// ? Formatting helpers
fn title(title: &str) -> colored::ColoredString {
    title.bold().green()
}

fn command(command: &str) -> colored::ColoredString {
    command.bold().cyan()
}

fn command_text(text: &str) -> colored::ColoredString {
    text.bold().yellow()
}

fn command_arg(arg: &str) -> colored::ColoredString {
    arg.cyan()
}
