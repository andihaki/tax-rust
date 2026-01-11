use std::io::{self, stdout};
use std::time::{Duration, Instant};

use crossterm::{
    event::{self},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};

use tax_core::App;

use crate::draw::draw;
use crate::events::handle_event;

fn setup_terminal() -> io::Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    Terminal::new(CrosstermBackend::new(stdout))
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()
}

pub struct Tui {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
    pub tick_rate: Duration,
}

impl Tui {
    pub fn new(tick_rate: Duration) -> io::Result<Self> {
        let terminal = setup_terminal()?;
        Ok(Self {
            terminal,
            tick_rate,
        })
    }

    pub fn run(&mut self, mut app: App) -> io::Result<()> {
        let mut last_tick = Instant::now();
        let mut should_run = true;

        while should_run {
            self.terminal.draw(|f| draw(f, &app))?;

            let timeout = self.tick_rate.saturating_sub(last_tick.elapsed());

            if event::poll(timeout)? {
                let event = event::read()?;
                should_run = handle_event(&mut app, &event);
            }

            if last_tick.elapsed() >= self.tick_rate {
                last_tick = Instant::now();
            }
        }

        Ok(())
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        let _ = restore_terminal(&mut self.terminal);
    }
}
