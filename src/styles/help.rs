use colored::{ColoredString, Colorize};

pub fn title(title: &str) -> ColoredString {
    title.bold().green()
}

pub fn command(command: &str) -> ColoredString {
    command.bold().cyan()
}

pub fn command_text(text: &str) -> ColoredString {
    text.bold().yellow()
}

pub fn command_arg(arg: &str) -> ColoredString {
    arg.cyan()
}
