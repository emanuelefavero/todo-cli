use colored::Colorize;

// 📢 PUBLIC ----------------------------------

// * Show the app usage instructions
pub fn usage() {
    println!("");
    println!("{}", title("Usage:"));
    println!("");
    println!("{}", title("Commands:"));
    println!("  {}                List all todos", command("todo"));
    println!(
        "  {} {}     Add a new todo",
        command("todo add"),
        command_text("\"text\"")
    );
    println!(
        "  {} {} {} Add a new todo at specific position",
        command("todo add"),
        command_text("\"text\""),
        command_arg("<n>")
    );
    println!("  {}             Remove the first todo", command("todo rm"));
    println!(
        "  {} {}         Remove a specific todo by number",
        command("todo rm"),
        command_arg("<n>")
    );
    println!(
        "  {}           Toggle the first todo completion status",
        command("todo done")
    );
    println!(
        "  {} {}       Toggle todo completion status",
        command("todo done"),
        command_arg("<n>")
    );
    println!("  {}          Remove all todos", command("todo clear"));
    println!(
        "  {}           Show this help message",
        command("todo help")
    );
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
