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

## Install from Homebrew

- To install Todo CLI using Homebrew, run the following commands:

  ```bash
  brew tap emanuelefavero/tap
  brew install todo
  ```

- Now you can run the `todo` command from anywhere in your terminal.

## Local Installation and Release

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

# Or release it using Homebrew (see below)
```

## Releasing the Application

[How to Release Your Own Version of `todo-cli`](RELEASE.md)

## Usage

The `todo` command is used to manage your todo list. Below are the available commands and their usage:

```bash
todo # Display the todo list
todo add "Todo text" # Add a new todo
todo rm # Remove first todo
todo rm <number> # Remove a specific todo by number
todo done # Mark the first todo as done
todo done <number> # Mark a specific todo as done
todo replace "New text" <number> # Replace a specific todo with new text
todo insert "Todo text" <number> # Insert a new todo at a specific position
todo edit # Edit a todo (interactive mode)
todo clear # Clear all todos
todo help # Display help information
```

### Examples

```bash
# Add a new todo
todo add "Buy groceries"

# Display the todo list
todo

# Mark first todo as done
todo done 1

# Remove the first todo
todo rm 1

# Clear all todos
todo clear
```

> Use the `help` command to see a list of all available commands and their descriptions
>
> You can also find aliases for commands in the help output, such as `todo a` for `todo add`.

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
