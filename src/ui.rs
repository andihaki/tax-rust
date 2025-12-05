use std::io::{self};
use std::time::{Duration, Instant};

use crossterm::event::{self};

use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span, ToText},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::App;

pub fn draw(frame: &mut Frame, app: &App) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(4),
            Constraint::Min(0),
        ])
        .split(frame.area());

    let title_block = Block::default()
        .borders(Borders::BOTTOM)
        .style(Style::default().fg(Color::Cyan));
    let title_text = "PJD Oleng Pajak\nInput gaji bulanan, lalu tekan Enter.".to_text();
    let title_paragraph = Paragraph::new(title_text)
        .block(title_block)
        .alignment(Alignment::Center);
    frame.render_widget(title_paragraph, main_layout[0]);

    let input_block = Block::default()
        .borders(Borders::ALL)
        .title("Gaji Bulanan / PKP (Penghasilan Kena Pajak)")
        .style(Style::default().fg(Color::Yellow));

    let input_text = Paragraph::new(app.input.as_str()).block(input_block);

    let cursor_x = main_layout[1].x + 1 + app.input.len() as u16;
    let cursor_y = main_layout[1].y + 1;
    frame.set_cursor_position((cursor_x, cursor_y));

    frame.render_widget(input_text, main_layout[1]);

    let results_block = Block::default()
        .borders(Borders::ALL)
        .title("Hasil perhitungan")
        .style(Style::default().fg(Color::White));

    let tax_due_formatted = format!("{:.2}", app.annual_tax_due);
    let tax_integer_part = tax_due_formatted.split('.').next().unwrap_or("0");
    let tax_decimal_part = tax_due_formatted.split('.').nth(1).unwrap_or("00");
    let tax_amount_u64 = tax_integer_part.parse::<u64>().unwrap_or(0);

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
                app.format_thousand_separator(app.monthly_income)
                    // @todo: masih gatel, repetitif
                    .unwrap_or_default(),
                Style::default().fg(Color::LightYellow),
            ),
        ]),
        Line::from(vec![
            Span::styled("Tahunan: ", Style::default().fg(Color::Gray)),
            Span::styled(
                app.format_thousand_separator(app.annual_income)
                    // @todo: masih gatel, repetitif
                    .unwrap_or_default(),
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
                    app.format_thousand_separator(tax_amount_u64)
                        // @todo: masih gatel, repetitif
                        .unwrap_or_default()
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

pub fn run_app(
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
