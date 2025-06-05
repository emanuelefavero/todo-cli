# Todo CLI

A command-line Todo List application built with Rust

## Overview

Todo CLI is a lightweight, fast, and easy-to-use task manager that allows you to manage your todos directly from the terminal. Perfect for developers and terminal enthusiasts who prefer working without leaving their command line environment.

## Features

&nbsp;

| _Feature_                             | _Description_                                      |
| ------------------------------------- | -------------------------------------------------- |
| ⚡ **&nbsp;Blazing Fast**             | Built with Rust for speed and efficiency           |
| 📦 **&nbsp;Small Binary Size**        | Lightweight and minimal                            |
| 📟 **&nbsp;Terminal-based Interface** | All interactions happen directly in your terminal  |
| ⚙️ **&nbsp;Dynamic Output**           | See changes in real-time as you run commands       |
| 🎨 **&nbsp;Colored Output**           | Syntax-highlighted tasks for better readability    |
| 💾 **&nbsp;Persistent Storage**       | Tasks are saved between sessions automatically     |
| 📝 **&nbsp;Add Tasks**                | Quickly add new tasks to your todo list            |
| 📋 **&nbsp;List Tasks**               | View all tasks with clear status indicators        |
| ✅ **&nbsp;Toggle Task Status**       | Mark tasks as done or undone with a single command |
| ❌ **&nbsp;Remove Tasks**             | Delete individual tasks with ease                  |
| ♻️ **&nbsp;Replace a Task**           | Update an existing task with new content           |
| ➕ **&nbsp;Insert Task at Position**  | Add a task at any specific index                   |
| 🧹 **&nbsp;Clear All Tasks**          | Wipe your task list in one command                 |

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)

### Building from Source

```bash
# Clone the repository
git clone https://github.com/emanuelefavero/todo-cli.git
cd todo-cli

# Build the project
cargo build --release

# Optional: Move the binary to a directory in your PATH so you can run it from anywhere
cp target/release/todo /usr/local/bin/
```

## Usage

The `todo` command is used to manage your todo list. Below are the available commands and their usage:

```bash
todo # List all todos
todo add "Todo text" # Add a new todo
todo rm # Remove the first todo
todo rm <number> # Remove a specific todo by number
todo done # Toggle the first todo completion status
todo done <number> # Toggle todo completion status
todo clear # Remove all todos
todo help # Show this help message
```

### Examples

```bash
# Add a new task
todo add "Buy groceries"

# List all tasks
todo

# Mark a task as done
todo done 1 # Mark first task as done

# Remove a specific task
todo rm 2 # Remove the second task

# Clear all tasks
todo clear
```

## Data Storage

All todos are stored as JSON in `~/.todo/todos.json`, making it easy to back up or sync your todos between computers.

## Author

[Emanuele Favero](https://github.com/emanuelefavero)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Resources

- [Rust](https://www.rust-lang.org/)
- [Colored Rust Library](https://github.com/colored-rs/colored)
- [Serde Rust JSON Library](https://serde.rs/)
- [dirs Rust Library](https://crates.io/crates/dirs)

## License

- [MIT](LICENSE.md)

&nbsp;

&nbsp;

[**Go To Top &nbsp; ⬆️**](#todo-cli)
