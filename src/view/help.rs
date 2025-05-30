use colored::Colorize;

// 📢 PUBLIC ----------------------------------

// * Show the app usage instructions
pub fn usage() {
    println!("");
    println!("{}", title("Usage:"));
    println!("");
    println!("{}", title("Commands:"));
    println!("  todo                List all todos");
    println!("  todo add \"text\"     Add a new todo");
    println!("  todo add \"text\" <n> Add a new todo at specific position");
    println!("  todo rm             Remove the first todo");
    println!("  todo rm <n>         Remove a specific todo by number");
    println!("  todo done           Toggle the first todo completion status");
    println!("  todo done <n>       Toggle todo completion status");
    println!("  todo clear          Remove all todos");
    println!("  todo help           Show this help message");
}

// 🔒 PRIVATE ---------------------------------

// ? Format title
fn title(title: &str) -> colored::ColoredString {
    title.bold().green()
}
