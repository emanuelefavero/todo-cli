use colored::Colorize;

pub fn title(title: &str) -> colored::ColoredString {
    title.bold().green()
}

pub fn command(command: &str) -> colored::ColoredString {
    command.bold().cyan()
}

pub fn command_text(text: &str) -> colored::ColoredString {
    text.bold().yellow()
}

pub fn command_arg(arg: &str) -> colored::ColoredString {
    arg.cyan()
}
