use std::io::stdout;

use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{Clear, ClearType},
};

// * Clear the terminal screen
pub fn clear() {
    let mut stdout = stdout();
    execute!(
        stdout,
        Clear(ClearType::All),
        MoveTo(0, 0), // ? Move cursor to top-left
    )
    .unwrap();
}
