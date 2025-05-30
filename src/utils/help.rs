use crate::models::help::HelpCommand;
use crate::styles::help::{command, command_arg, command_text};
use colored::ColoredString;

// * Calculate the maximum length of all command combinations for proper spacing
pub fn calculate_max_command_length(commands: &[HelpCommand]) -> usize {
    commands
        .iter()
        .map(|cmd| calculate_components_length(cmd))
        .max() // Find the maximum length
        .unwrap_or(0) // Default to 0 if no commands are present
}

// * Format a command's components (command, text, arg)
pub fn format_command_components(cmd: &HelpCommand) -> (ColoredString, String, String) {
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

// * Calculate the length of a command's components for spacing
pub fn calculate_components_length(cmd: &HelpCommand) -> usize {
    cmd.command.len()
        + cmd.command_text.as_ref().map_or(0, |t| t.len() + 1)
        + cmd.command_arg.as_ref().map_or(0, |a| a.len() + 1)
}
