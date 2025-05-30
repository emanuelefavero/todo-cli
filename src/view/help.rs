use colored::Colorize;
use crate::models::help::HelpCommand;

// 📢 PUBLIC ----------------------------------

// * Show the app usage instructions
pub fn usage() {
    // Define all commands
    let commands = vec![
        HelpCommand {
            command: "todo".to_string(),
            description: "List all todos".to_string(),
            command_text: None,
            command_arg: None,
        },
        HelpCommand {
            command: "todo add".to_string(),
            description: "Add a new todo".to_string(),
            command_text: Some("\"text\"".to_string()),
            command_arg: None,
        },
        HelpCommand {
            command: "todo add".to_string(),
            description: "Add a new todo at specific position".to_string(),
            command_text: Some("\"text\"".to_string()),
            command_arg: Some("<n>".to_string()),
        },
        HelpCommand {
            command: "todo rm".to_string(),
            description: "Remove the first todo".to_string(),
            command_text: None,
            command_arg: None,
        },
        HelpCommand {
            command: "todo rm".to_string(),
            description: "Remove a specific todo by number".to_string(),
            command_text: None,
            command_arg: Some("<n>".to_string()),
        },
        HelpCommand {
            command: "todo done".to_string(),
            description: "Toggle the first todo completion status".to_string(),
            command_text: None,
            command_arg: None,
        },
        HelpCommand {
            command: "todo done".to_string(),
            description: "Toggle todo completion status".to_string(),
            command_text: None,
            command_arg: Some("<n>".to_string()),
        },
        HelpCommand {
            command: "todo clear".to_string(),
            description: "Remove all todos".to_string(),
            command_text: None,
            command_arg: None,
        },
        HelpCommand {
            command: "todo help".to_string(),
            description: "Show this help message".to_string(),
            command_text: None,
            command_arg: None,
        },
    ];

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
