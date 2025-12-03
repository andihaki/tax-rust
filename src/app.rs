use std::num::IntErrorKind;

use crossterm::event::{Event, KeyCode, KeyEventKind};

use crate::constants::*;

#[derive(Default)]
pub struct App {
    pub input: String,
    pub monthly_income: u64,
    pub annual_income: u64,
    pub annual_tax_due: f64,
    pub tax_bracket_info: String,
}

impl App {
    pub fn new() -> App {
        App {
            tax_bracket_info: "".to_string(),
            ..Default::default()
        }
    }

    pub fn calculate_tax(&mut self) {
        match self.input.parse::<u64>() {
            Ok(monthly_income) => {
                self.monthly_income = monthly_income;
                self.annual_income = monthly_income;

                let income = self.annual_income;
                let rate: f64;
                let bracket_desc: &str;

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
                    bracket_desc = "Golongan Gajhi Nyuyok (500 juta - 5 miliar rupiah)";
                } else {
                    rate = RATE_5;
                    bracket_desc = "Golongan Sultan (> 5 miliar rupiah)";
                }

                self.annual_tax_due = (income as f64) * rate;

                let rate_percentage = format!("{}%", rate * 100.0);
                // Update the display information
                self.tax_bracket_info = format!(
                    "Penghasilan termasuk {} dengan pajak {}.",
                    bracket_desc, rate_percentage
                );
            }
            Err(e) => {
                self.annual_income = 0;
                self.monthly_income = 0;
                self.annual_tax_due = 0.0;

                if matches!(*e.kind(), IntErrorKind::PosOverflow) {
                    self.tax_bracket_info = "Error: Error: Nilai input terlalu besar.".to_string();
                } else {
                    self.tax_bracket_info = "Error: format angka tidak valid.".to_string();
                }
            }
        }
    }

    pub fn handle_event(&mut self, event: &Event) -> bool {
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
                if self.input.len() < MAX_INPUT_LENGTH {
                    self.input.push(c);
                }
                true
            }
            _ => true,
        }
    }

    pub fn format_income_thousand_separator(&self, amount: u64) -> String {
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
