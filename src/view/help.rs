use colored::Colorize;

// 📢 PUBLIC ----------------------------------

// * Show the app usage instructions
pub fn usage() {
    println!("");
    println!("{}", title("Usage:"));
    println!("");
    println!("{}", title("Commands:"));
    println!("  {}                List all todos", command("todo"));
    println!("  {}     Add a new todo", command("todo add \"text\""));
    println!(
        "  {} Add a new todo at specific position",
        command("todo add \"text\" <n>")
    );
    println!("  {}             Remove the first todo", command("todo rm"));
    println!(
        "  {}         Remove a specific todo by number",
        command("todo rm <n>")
    );
    println!(
        "  {}           Toggle the first todo completion status",
        command("todo done")
    );
    println!(
        "  {}       Toggle todo completion status",
        command("todo done <n>")
    );
    println!("  {}          Remove all todos", command("todo clear"));
    println!(
        "  {}           Show this help message",
        command("todo help")
    );
}

// 🔒 PRIVATE ---------------------------------

// ? Format title
fn title(title: &str) -> colored::ColoredString {
    title.bold().green()
}

// ? Format command
fn command(command: &str) -> colored::ColoredString {
    command.bold().cyan()
}
