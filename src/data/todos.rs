use std::fs;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

use colored::Colorize;
use rustyline::DefaultEditor;

use crate::models::todo::Todo;
use crate::utils::todos::{validate_index, validate_index_on_add};
use crate::view;

// 📢 PUBLIC ----------------------------------

// * Reads todos from a JSON file
pub fn read() -> Result<Vec<Todo>, Error> {
    let path = file_path();

    if !path.exists() {
        fs::write(&path, "[]").expect("Could not create todo file");
        return Ok(Vec::new());
    }

    let content = fs::read_to_string(&path)?;
    let todos: Vec<Todo> = serde_json::from_str(&content).map_err(|e| {
        Error::new(
            ErrorKind::InvalidData,
            format!("Could not parse todos: {}", e),
        )
    })?;

    Ok(todos)
}

// * Writes todos to a JSON file
pub fn write(todos: &Vec<Todo>) -> Result<(), Error> {
    let path = file_path();
    let content = serde_json::to_string_pretty(todos).map_err(|e| {
        Error::new(
            ErrorKind::InvalidData,
            format!("Could not serialize todos: {}", e),
        )
    })?;

    fs::write(path, content)?;
    Ok(())
}

// * Clears all todos from the list
pub fn clear() -> Result<(), Error> {
    let todos = read()?;

    view::todos::title();

    if todos.is_empty() {
        view::todos::empty();
        return Ok(());
    }

    // Write an empty array to clear all todos
    write(&Vec::new())?;
    println!("🗑️  All todos cleared");

    Ok(())
}

// * Adds a new todo to the list, optionally at a specific index
pub fn add(text: &str, index: Option<usize>) -> Result<(), Error> {
    let mut todos = read()?;

    // Create the new todo
    let new_todo = Todo {
        text: text.to_string(),
        done: false,
    };

    // Either insert at a specific index or add to the end
    // TIP: `match index` is used to handle both cases
    match index {
        // If an index is provided, insert at that position
        Some(idx) => {
            // Check if the index is valid
            validate_index_on_add(idx, &todos)?;

            // Insert the new todo at the specified index
            todos.insert(idx - 1, new_todo);
        }
        // If no index is provided, add to the end of the list
        None => {
            // Add to the end of the list (default behavior)
            todos.push(new_todo);
        }
    }

    write(&todos)?;

    // Show the updated list with the new todo highlighted
    view::todos::added(index)?;

    Ok(())
}

// * Removes a todo from the list by index
pub fn remove(index: usize) -> Result<(), Error> {
    let mut todos = read()?;

    // Check if the todo list is empty first
    if todos.is_empty() {
        view::todos::title();
        view::todos::empty();
        return Ok(());
    }

    validate_index(index, &todos)?;

    let todo = todos.remove(index - 1);
    write(&todos)?;

    // Show the updated list with the removed todo
    view::todos::removed(index, &todo)?;

    Ok(())
}

// * Toggles the done status of a todo by index
pub fn toggle(index: usize) -> Result<(), Error> {
    let mut todos = read()?;

    // Check if the todo list is empty first
    if todos.is_empty() {
        view::todos::title();
        view::todos::empty();
        return Ok(());
    }

    validate_index(index, &todos)?;

    // Toggle the done status
    todos[index - 1].done = !todos[index - 1].done;

    write(&todos)?;

    // Show the updated list
    view::todos::toggled(index)?;

    Ok(())
}

// * Replaces the text of a todo at a specific index
pub fn replace(index: usize, new_text: &str) -> Result<(), Error> {
    let mut todos = read()?;

    // Check if the todo list is empty first
    if todos.is_empty() {
        view::todos::title();
        view::todos::empty();
        return Ok(());
    }

    validate_index(index, &todos)?;

    // Save the old todo text before replacing
    let old_text = todos[index - 1].text.clone();

    // Replace the todo at the specified index
    todos[index - 1].text = new_text.to_string();

    write(&todos)?;

    // Show the updated list with the replaced todo, passing both old and new text
    view::todos::replaced(index, &old_text, new_text)?;

    Ok(())
}

// * Edits a todo at a specific index or allows interactive selection
pub fn edit(index: Option<usize>) -> Result<(), Error> {
    use crossterm::{
        cursor::{Hide, MoveTo, Show},
        event::{self, Event, KeyCode, KeyEventKind},
        execute,
        style::{Color, ResetColor, SetBackgroundColor, SetForegroundColor},
        terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
    };
    use std::io::{Write, stdout};

    let mut todos = read()?;

    // Check if the todo list is empty first
    if todos.is_empty() {
        view::todos::title();
        view::todos::empty();
        return Ok(());
    }

    // If no index was provided, start with the first todo selected
    let mut selected_index = match index {
        Some(idx) => {
            validate_index(idx, &todos)?;
            idx - 1 // Convert to 0-based index for internal use
        }
        None => 0, // Start with the first todo selected
    };

    // Enter raw mode for direct key handling
    enable_raw_mode()?;

    let mut stdout = stdout();

    // Hide cursor during navigation
    execute!(stdout, Hide)?;

    // Interactive selection loop
    'outer: loop {
        // Clear screen and draw the todo list
        execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;

        // Display title with exact positioning like in title()
        execute!(stdout, MoveTo(0, 1))?; // Position at top with one blank line
        execute!(stdout, MoveTo(0, 1))?; // Make sure we're at column 0
        write!(stdout, "📝 {}", "Todo List ".bold())?;
        write!(stdout, "{}", "Edit Mode".yellow().italic())?;
        execute!(stdout, MoveTo(0, 2))?; // Next line, column 0
        write!(stdout, "{}", "──────────────────────".dimmed())?;

        execute!(stdout, MoveTo(0, 3))?; // Next line, column 0

        // Display todos with the selected one highlighted - enforce positioning at column 0
        for (i, todo) in todos.iter().enumerate() {
            let index_str = format!("{}", i + 1);
            let status = if todo.done {
                "✓".green()
            } else {
                "○".yellow()
            };

            execute!(stdout, MoveTo(0, 3 + i as u16))?; // Position at start of line

            // Highlight selected todo
            if i == selected_index {
                execute!(
                    stdout,
                    SetBackgroundColor(Color::Blue),
                    SetForegroundColor(Color::White)
                )?;
                write!(stdout, "{} {} {}", index_str.purple(), status, todo.text)?;
                execute!(stdout, ResetColor)?;
            } else {
                write!(stdout, "{} {} {}", index_str.purple(), status, todo.text)?;
            }
            writeln!(stdout)?; // End the line after writing the todo
        }

        // Add navigation instructions at the bottom
        execute!(stdout, MoveTo(0, 3 + todos.len() as u16 + 1))?; // Position after todos with a blank line
        write!(
            stdout,
            "{}",
            "Use ↑/↓ to navigate, Enter to edit, Esc to cancel"
                .italic()
                .dimmed()
        )?;

        stdout.flush()?;

        // Handle key events
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Up => {
                        if selected_index > 0 {
                            selected_index -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if selected_index < todos.len() - 1 {
                            selected_index += 1;
                        }
                    }
                    KeyCode::Enter => {
                        // Exit raw mode before editing
                        disable_raw_mode()?;
                        execute!(stdout, Show)?;

                        // Get current text
                        let current_text = &todos[selected_index].text.clone();

                        // Clear screen and prepare for text input
                        execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;

                        // Use rustyline for input with pre-populated text
                        let mut rl = DefaultEditor::new().map_err(|e| {
                            Error::new(
                                ErrorKind::Other,
                                format!("Failed to initialize editor: {}", e),
                            )
                        })?;

                        let prompt_text = format!(
                            "{} {}{} ",
                            "Edit todo".yellow(),
                            (selected_index + 1).to_string().magenta(),
                            ":".yellow()
                        );

                        // Pre-populate the input with the current todo text
                        let new_text = rl
                            .readline_with_initial(&prompt_text, (current_text, ""))
                            .map_err(|e| {
                            Error::new(ErrorKind::Other, format!("Failed to read input: {}", e))
                        })?;

                        let new_text = new_text.trim();

                        // If the user just presses Enter without entering text, keep the original text
                        if !new_text.is_empty() {
                            // Save the old text before replacing
                            let old_text = todos[selected_index].text.clone();

                            // Replace the todo text
                            todos[selected_index].text = new_text.to_string();

                            // Write changes to file
                            write(&todos)?;

                            // Show the updated list with the replaced todo
                            view::todos::replaced(selected_index + 1, &old_text, new_text)?;
                        } else {
                            println!("No changes made.");
                        }

                        // Exit the loop after editing
                        break 'outer;
                    }
                    KeyCode::Esc => {
                        // Cancel editing
                        break 'outer;
                    }
                    _ => {}
                }
            }
        }
    }

    // Ensure raw mode is disabled and cursor is shown before returning
    disable_raw_mode()?;
    execute!(stdout, Show)?;

    Ok(())
}

// 🔒 PRIVATE ---------------------------------

// ? Creates the file path for the todo file
fn file_path() -> PathBuf {
    // Get the home directory
    let mut path = dirs::home_dir().expect("Could not find home directory");

    // NOTE: Choose the directory where the todo file will be stored
    path.push(".todo");

    // Create the directory if it doesn't exist
    fs::create_dir_all(&path).expect("Could not create directory: .todo/");

    path.push("todos.json"); // Append the filename to the path
    path // Return the full path to the todo file
}
