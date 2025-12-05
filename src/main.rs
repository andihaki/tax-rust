mod app;
mod constants;
mod ui;

use app::App;

use std::io;
use std::time::Duration;

use crate::ui::{restore_terminal, run_app, setup_terminal};

fn main() -> io::Result<()> {
    let mut terminal = setup_terminal()?;

    let app = App::new();

    let tick_rate = Duration::from_millis(16); // ~60fps
    let res = run_app(&mut terminal, app, tick_rate);

    restore_terminal(&mut terminal)?;

    res
}
