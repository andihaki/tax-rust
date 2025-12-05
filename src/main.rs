mod app;
mod constants;
mod ui;

use app::App;

use std::io::{self, stdout};
use std::time::Duration;

use crate::ui::run_app;

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    crossterm::{
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen},
    },
};

fn main() -> io::Result<()> {
    // 1. Setup terminal
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;

    // 2. Initialize App State
    let app = App::new();

    // 3. Main TUI Loop
    let tick_rate = Duration::from_millis(16); // ~60fps

    // Run the main application loop
    let res = run_app(&mut terminal, app, tick_rate);

    // 4. Restore terminal state
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    res
}
