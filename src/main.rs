mod app;
mod constants;

use app::App;

use std::io::{self, stdout};
use std::time::{Duration, Instant};

use crossterm::{
    event::{self},
    terminal::{disable_raw_mode, enable_raw_mode},
};

use ratatui::text::ToText;
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    crossterm::{
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen},
    },
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

// --- TUI DRAWING ---

/// Draws the application UI.
fn ui(frame: &mut Frame, app: &App) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Length(4), // Input
            Constraint::Min(0),    // Results
        ])
        .split(frame.area());

    // 1. Title Block
    let title_block = Block::default()
        .borders(Borders::BOTTOM)
        .style(Style::default().fg(Color::Cyan));
    let title_text = "PJD Oleng Pajak\nInput gaji bulanan, lalu tekan Enter.".to_text();
    let title_paragraph = Paragraph::new(title_text)
        .block(title_block)
        .alignment(ratatui::layout::Alignment::Center);
    frame.render_widget(title_paragraph, main_layout[0]);

    // 2. Input Block
    let input_block = Block::default()
        .borders(Borders::ALL)
        .title("Gaji Bulanan / PKP (Penghasilan Kena Pajak)")
        .style(Style::default().fg(Color::Yellow));

    // Display the current input string
    let input_text = Paragraph::new(app.input.as_str()).block(input_block);

    // Position the cursor at the end of the input string for visual feedback
    let cursor_x = main_layout[1].x + 1 + app.input.len() as u16;
    let cursor_y = main_layout[1].y + 1;
    frame.set_cursor_position((cursor_x, cursor_y));

    frame.render_widget(input_text, main_layout[1]);

    // 3. Results Block
    let results_block = Block::default()
        .borders(Borders::ALL)
        .title("Hasil perhitungan")
        .style(Style::default().fg(Color::White));

    let tax_due_formatted = format!("{:.2}", app.annual_tax_due);
    // Safely extract the integer part of the tax for formatting
    let tax_integer_part = tax_due_formatted.split('.').next().unwrap_or("0");
    let tax_decimal_part = tax_due_formatted.split('.').nth(1).unwrap_or("00");
    let tax_amount_u64 = tax_integer_part.parse::<u64>().unwrap_or(0);

    // Results content
    let results_text = vec![
        Line::from(Span::styled(
            "--- Detail Gaji---",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(vec![
            Span::styled("Bulanan: ", Style::default().fg(Color::Gray)),
            Span::styled(
                app.format_income_thousand_separator(app.monthly_income),
                Style::default().fg(Color::LightYellow),
            ),
        ]),
        Line::from(vec![
            Span::styled("Tahunan: ", Style::default().fg(Color::Gray)),
            Span::styled(
                app.format_income_thousand_separator(app.annual_income),
                Style::default().fg(Color::LightYellow),
            ),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "--- Detail Pajak ---",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            &app.tax_bracket_info,
            Style::default().fg(Color::Magenta),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "Total Pajak Tahunan terhitung: ",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!(
                    "RP {}",
                    app.format_income_thousand_separator(tax_amount_u64)
                ),
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!(".{}", tax_decimal_part),
                Style::default().fg(Color::Red),
            ),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "Tekan 'q' atau 'Esc' untuk exit.",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    let results_paragraph = Paragraph::new(results_text).block(results_block);
    frame.render_widget(results_paragraph, main_layout[2]);
}

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
        terminal.draw(|f| ui(f, &app))?;

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
