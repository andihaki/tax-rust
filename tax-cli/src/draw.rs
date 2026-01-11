use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::constants::{INPUT_HEIGHT, TITLE_HEIGHT};
use crate::models::TaxBreakdown;
use tax_core::App;

pub fn draw(frame: &mut Frame, app: &App) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(TITLE_HEIGHT),
            Constraint::Length(INPUT_HEIGHT),
            Constraint::Min(0),
        ])
        .split(frame.area());

    render_title(frame, main_layout[0]);
    render_input(frame, main_layout[1], app);
    render_results(frame, main_layout[2], app);
}

fn render_title(frame: &mut Frame, area: Rect) {
    let title_block = Block::default()
        .borders(Borders::BOTTOM)
        .style(Style::default().fg(Color::Cyan));

    let title_text = "PJD Oleng Pajak\nInput gaji bulanan, lalu tekan Enter.";
    let title_paragraph = Paragraph::new(title_text)
        .block(title_block)
        .alignment(Alignment::Center);

    frame.render_widget(title_paragraph, area);
}

fn render_input(frame: &mut Frame, area: Rect, app: &App) {
    let input_block = Block::default()
        .borders(Borders::ALL)
        .title("Gaji Bulanan / PKP (Penghasilan Kena Pajak)")
        .style(Style::default().fg(Color::Yellow));

    let input_text = Paragraph::new(app.input.as_str()).block(input_block);

    let cursor_x = area.x + 1 + app.input.len() as u16;
    let cursor_y = area.y + 1;
    frame.set_cursor_position((cursor_x, cursor_y));

    frame.render_widget(input_text, area);
}

fn render_results(frame: &mut Frame, area: Rect, app: &App) {
    let results_block = Block::default()
        .borders(Borders::ALL)
        .title("Hasil perhitungan")
        .style(Style::default().fg(Color::White));

    let tax_parts = TaxBreakdown::from(app.annual_tax_due);

    let results_text = vec![
        create_section_header("--- Detail Gaji ---"),
        create_income_line(
            "Bulanan: ",
            app.format_thousand_separator(app.monthly_income),
        ),
        create_income_line(
            "Tahunan: ",
            app.format_thousand_separator(app.annual_income),
        ),
        Line::from(""),
        create_section_header("--- Detail Pajak ---"),
        Line::from(Span::styled(
            &app.tax_bracket_info,
            Style::default().fg(Color::Magenta),
        )),
        Line::from(""),
        create_tax_line(&app, &tax_parts),
        Line::from(""),
        Line::from(Span::styled(
            "Tekan 'q' atau 'Esc' untuk exit.",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    let results_paragraph = Paragraph::new(results_text).block(results_block);
    frame.render_widget(results_paragraph, area);
}

fn create_section_header(text: &str) -> Line<'static> {
    Line::from(Span::styled(
        text.to_string(), // Convert to owned String
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD),
    ))
}

fn create_income_line(label: &str, amount: String) -> Line<'static> {
    Line::from(vec![
        Span::styled(label.to_string(), Style::default().fg(Color::Gray)),
        Span::styled(amount, Style::default().fg(Color::LightYellow)),
    ])
}

fn create_tax_line(app: &App, tax_parts: &TaxBreakdown) -> Line<'static> {
    Line::from(vec![
        Span::styled(
            "Total Pajak Tahunan terhitung: ",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!("RP {}", app.format_thousand_separator(tax_parts.integer)),
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!(".{}", tax_parts.decimal),
            Style::default().fg(Color::Red),
        ),
    ])
}
