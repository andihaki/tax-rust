use crate::constants::{INPUT_ERROR_MSG, MAX_INPUT_LENGTH};
use crossterm::event::{Event, KeyCode, KeyEventKind};
use tax_core::App;

pub fn handle_event(app: &mut App, event: &Event) -> bool {
    let Event::Key(key) = event else {
        return true;
    };

    if key.kind != KeyEventKind::Press {
        return true;
    }

    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => false,
        KeyCode::Enter => {
            app.calculate_tax();
            true
        }
        KeyCode::Backspace => {
            app.input.pop();
            true
        }
        KeyCode::Char(c) if c.is_ascii_digit() => {
            handle_digit_input(app, c);
            true
        }
        _ => true,
    }
}

fn handle_digit_input(app: &mut App, c: char) {
    if app.input.len() < MAX_INPUT_LENGTH {
        app.input.push(c);
    } else {
        app.tax_bracket_info = format!("{}. {} digit", INPUT_ERROR_MSG, MAX_INPUT_LENGTH);
    }
}
