use std::io::{self, stdout};
use std::num::IntErrorKind;
use std::time::{Duration, Instant};

// Import event-related items and raw mode functions directly from crossterm
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode},
};

use ratatui::text::ToText;
// Import ratatui components, and crucially, the 'execute' macro and
// the terminal commands (EnterAlternateScreen, LeaveAlternateScreen)
// via ratatui's re-export to satisfy the trait bounds.
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

// Type alias for the terminal backend
type TuiTerminal = Terminal<CrosstermBackend<io::Stdout>>;

// --- CONSTANTS FOR TAX CALCULATION ---

// Define the upper limits (in RP) for each tax bracket
const BRACKET_1_LIMIT: u64 = 50_000_000;
const BRACKET_2_LIMIT: u64 = 250_000_000;
const BRACKET_3_LIMIT: u64 = 500_000_000;
const BRACKET_4_LIMIT: u64 = 5_000_000_000;

// Define the tax rates for each bracket
const RATE_1: f64 = 0.05; // 5%
const RATE_2: f64 = 0.15; // 15%
const RATE_3: f64 = 0.25; // 25%
const RATE_4: f64 = 0.30; // 30%
const RATE_5: f64 = 0.35; // 35%

const MAX_INPUT_LENGTH: usize = 18;

// --- APPLICATION STATE ---

#[derive(Default)]
struct App {
    input: String,
    monthly_pkp: u64,
    annual_pkp: u64,
    annual_tax_due: f64,
    tax_bracket_info: String,
}

impl App {
    fn new() -> App {
        App {
            tax_bracket_info: "".to_string(),
            ..Default::default()
        }
    }

    /// Handles the logic for calculating tax and updating the state.
    fn calculate_tax(&mut self) {
        // Attempt to parse the input string into a u64
        match self.input.parse::<u64>() {
            Ok(monthly_income) => {
                self.monthly_pkp = monthly_income;
                // Note: The annual_pkp calculation (monthly_income * 12) can also overflow u64,
                // but by limiting the input length to 18 digits, we ensure the product
                // will remain within the u64 range (max monthly input ~ 1.5e18, max u64 ~ 18.4e18)
                self.annual_pkp = monthly_income.saturating_mul(12);

                let income = self.annual_pkp;
                let rate: f64;
                let bracket_desc: &str;

                // Determine the highest applicable bracket (Single-Tier Lookup)
                if income == 0 {
                    rate = 0.0;
                    bracket_desc = "Tidak ada penghasilan kena pajak";
                } else if income <= BRACKET_1_LIMIT {
                    rate = RATE_1;
                    bracket_desc = "Golongan 1 (0 - 50 juta rupiah)";
                } else if income <= BRACKET_2_LIMIT {
                    rate = RATE_2;
                    bracket_desc = "Golongan 2 (50 - 250 juta rupiah)";
                } else if income <= BRACKET_3_LIMIT {
                    rate = RATE_3;
                    bracket_desc = "Golongan 3 (250 - 500 juta rupiah)";
                } else if income <= BRACKET_4_LIMIT {
                    rate = RATE_4;
                    bracket_desc = "Golongan Nyuyok (500 juta - 5 miliar rupiah)";
                } else {
                    rate = RATE_5;
                    bracket_desc = "Golongan Sultan (> 5 miliar rupiah)";
                }

                self.annual_tax_due = (income as f64) * rate;

                let rate_percentage = format!("{}%", rate * 100.0);
                // Update the display information
                self.tax_bracket_info = format!(
                    "Penghasilan masuk {} dengan pajak {}.",
                    bracket_desc, rate_percentage
                );
            }
            Err(e) => {
                self.annual_pkp = 0;
                self.monthly_pkp = 0;
                self.annual_tax_due = 0.0;

                // Check if the error is specifically due to a positive overflow
                if matches!(*e.kind(), IntErrorKind::PosOverflow) {
                    self.tax_bracket_info = "Error: Error: Nilai input terlalu besar.".to_string();
                } else {
                    // General parsing error (e.g., non-digit if not caught by handle_event)
                    self.tax_bracket_info = "Error: format angka tidak valid.".to_string();
                }
            }
        }
    }

    fn handle_event(&mut self, event: &Event) -> bool {
        let Event::Key(key) = event else {
            return true;
        };

        if key.kind != KeyEventKind::Press {
            return true;
        }

        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => false,
            KeyCode::Enter => {
                self.calculate_tax();
                true
            }
            KeyCode::Backspace => {
                self.input.pop();
                true
            }
            KeyCode::Char(c) if c.is_ascii_digit() => {
                self.handle_digit_input(c);
                true
            }
            _ => true,
        }
    }

    fn handle_digit_input(&mut self, c: char) {
        if self.input.len() < MAX_INPUT_LENGTH {
            self.input.push(c);
        } else {
            self.tax_bracket_info = format!("Input diblokir: Maksimal {} digit.", MAX_INPUT_LENGTH);
        }
    }

    /// Helper function to format the u64 amount into a string with thousands separators.
    fn format_income(&self, amount: u64) -> String {
        amount
            .to_string()
            .as_bytes()
            .rchunks(3)
            .rev()
            .map(|chunk| String::from_utf8(chunk.to_vec()).unwrap())
            .collect::<Vec<String>>()
            .join(".")
    }
}

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
                app.format_income(app.monthly_pkp),
                Style::default().fg(Color::LightYellow),
            ),
        ]),
        Line::from(vec![
            Span::styled("Tahunan: ", Style::default().fg(Color::Gray)),
            Span::styled(
                app.format_income(app.annual_pkp),
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
                format!("RP {}", app.format_income(tax_amount_u64)),
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
fn run_app(terminal: &mut TuiTerminal, mut app: App, tick_rate: Duration) -> io::Result<()> {
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
