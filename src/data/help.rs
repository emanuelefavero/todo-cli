use crate::models::help::HelpCommand;

// * Define all help commands
pub fn get_commands() -> Vec<HelpCommand> {
    vec![
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
    ]
}
