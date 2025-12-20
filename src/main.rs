use std::io;
use std::time::Duration;

use tax_cli::{restore_terminal, run_app, setup_terminal};
use tax_core::App;

fn main() -> io::Result<()> {
    let mut terminal = setup_terminal()?;

    let app = App::new();

    let tick_rate = Duration::from_millis(16); // ~60fps
    let res = run_app(&mut terminal, app, tick_rate);

    restore_terminal(&mut terminal)?;

    res
}
