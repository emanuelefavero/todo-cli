// * Show the app usage instructions
pub fn usage() {
    println!("Usage:");
    println!("  todo - List all todos");
    println!("  todo add \"Todo text\" - Add a new todo");
    println!("  todo add \"Todo text\" <number> - Add a new todo at specific position");
    println!("  todo rm - Remove the first todo");
    println!("  todo rm <number> - Remove a specific todo by number");
    println!("  todo done - Toggle the first todo completion status");
    println!("  todo done <number> - Toggle todo completion status");
    println!("  todo clear - Remove all todos");
    println!("  todo help - Show this help message");
}
