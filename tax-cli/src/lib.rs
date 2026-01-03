mod constants;

use std::io::{self, stdout};
use std::time::{Duration, Instant};

use crossterm::event::{Event, KeyCode, KeyEventKind};
use crossterm::{
    event::{self},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    crossterm::{
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen},
    },
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use tax_core::App;

use crate::constants::MAX_INPUT_LENGTH;

fn setup_terminal() -> io::Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    Terminal::new(CrosstermBackend::new(stdout))
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

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()
}

fn draw(frame: &mut Frame, app: &App) {
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
    let title_text = "PJD Oleng Pajak\nInput gaji bulanan, lalu tekan Enter.";
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

    let tax_parts = TaxBreakdown::from(app.annual_tax_due);

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
                app.format_thousand_separator(app.monthly_income),
                Style::default().fg(Color::LightYellow),
            ),
        ]),
        Line::from(vec![
            Span::styled("Tahunan: ", Style::default().fg(Color::Gray)),
            Span::styled(
                app.format_thousand_separator(app.annual_income),
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
                format!("RP {}", app.format_thousand_separator(tax_parts.integer)),
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!(".{}", tax_parts.decimal),
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

fn handle_event(app: &mut App, event: &Event) -> bool {
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
            if app.input.len() < MAX_INPUT_LENGTH {
                app.input.push(c);
            } else {
                app.tax_bracket_info = format!("Gaji anda diluar nurul. {} digit", MAX_INPUT_LENGTH)
            }
            true
        }
        _ => true,
    }
}

struct TaxBreakdown {
    integer: u64,
    decimal: String,
}

impl TaxBreakdown {
    fn from(amount: f64) -> Self {
        let total_cents = (amount * 100.0) as u64;
        let integer = total_cents / 100;
        let decimal = format!("{:02}", total_cents % 100);
        Self { integer, decimal }
    }
}
