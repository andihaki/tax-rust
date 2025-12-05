mod app;
mod constants;
mod ui;

use app::App;
use ui::draw;

use std::io::{self, stdout};
use std::time::{Duration, Instant};

use crossterm::{
    event::{self},
    terminal::{disable_raw_mode, enable_raw_mode},
};

use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    crossterm::{
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen},
    },
};

/// Main application event loop
fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    let mut should_run = true;

    while should_run {
        // Draw the UI
        terminal.draw(|f| draw(f, &app))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());

        // Handle Events
        if event::poll(timeout)? {
            let event = event::read()?;
            should_run = app.handle_event(&event);
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }

    Ok(())
}

// --- MAIN EXECUTION ---

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
