use std::io;
use std::time::Duration;

use tax_cli::Tui;
use tax_core::App;

fn main() -> io::Result<()> {
    let app = App::new();
    let tick_rate = Duration::from_millis(16); // ~60fps

    let mut tui = Tui::new(tick_rate)?;
    tui.run(app)
}
