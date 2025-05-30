use colored::Colorize;
use crate::data::help::get_commands;

// 📢 PUBLIC ----------------------------------

// * Show the app usage instructions
pub fn usage() {
    // Get all commands
    let commands = get_commands();

    // Calculate the max length of command components
    let mut max_length = 0;
    for cmd in &commands {
        let cmd_length = cmd.command.len() 
            + cmd.command_text.as_ref().map_or(0, |t| t.len() + 1) // +1 for space
            + cmd.command_arg.as_ref().map_or(0, |a| a.len() + 1); // +1 for space
        
        if cmd_length > max_length {
            max_length = cmd_length;
        }
    }

    // Print the header
    println!("");
    println!("{}", title("Usage:"));
    println!("");
    println!("{}", title("Commands:"));

    // Print each command with the correct spacing
    for cmd in &commands {
        // Format command components
        let cmd_text = match &cmd.command_text {
            Some(text) => format!(" {}", command_text(text)),
            None => String::new(),
        };

        let cmd_arg = match &cmd.command_arg {
            Some(arg) => format!(" {}", command_arg(arg)),
            None => String::new(),
        };

        // Calculate the length of the command components for spacing
        let cmd_components_length = cmd.command.len() 
            + cmd.command_text.as_ref().map_or(0, |t| t.len() + 1)
            + cmd.command_arg.as_ref().map_or(0, |a| a.len() + 1);

        // Calculate spaces needed
        let spaces = " ".repeat(max_length - cmd_components_length + 1);

        println!(
            "  {}{}{}{} {}",
            command(&cmd.command),
            cmd_text,
            cmd_arg,
            spaces,
            cmd.description
        );
    }
}

// 🔒 PRIVATE ---------------------------------

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
