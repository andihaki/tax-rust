use crossterm::event::{Event, KeyCode, KeyEventKind};
use std::{num::IntErrorKind, string::FromUtf8Error};

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

    fn calculate_tax(&mut self) {
        match self.input.parse::<u64>() {
            Ok(monthly_income) => {
                self.monthly_income = monthly_income;
                self.annual_income = monthly_income * 12;

                let income = self.annual_income;
                let (rate, bracket_desc) = App::get_tax_bracket(income);

                self.annual_tax_due = (income as f64) * rate;

                let rate_percentage = format!("{}%", rate * 100.0);
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
                    self.tax_bracket_info = "Error: Gaji boss satu ni diluar nurul.".to_string();
                } else {
                    // seems should never called because guarded by if c.is_ascii_digit()
                    self.tax_bracket_info = "Error: hmmmm.".to_string();
                }
            }
        }
    }

    fn get_tax_bracket(income: u64) -> (f64, &'static str) {
        match income {
            0 => (0.0, "Tyduck kena pajak"),
            _ if income <= BRACKET_1_LIMIT => (RATE_1, "Golongan 1 (0 - 50 juta rupiah)"),
            _ if income <= BRACKET_2_LIMIT => (RATE_2, "Golongan 2 (50 - 250 juta rupiah)"),
            _ if income <= BRACKET_3_LIMIT => (RATE_3, "Golongan 3 (250 - 500 juta rupiah)"),
            _ if income <= BRACKET_4_LIMIT => {
                (RATE_4, "Golongan Gajhi Nyuyok (500 juta - 5 miliar rupiah)")
            }
            _ => (RATE_5, "Golongan Sultan (> 5 miliar rupiah)"),
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
                self.input.push(c);
                true
            }
            _ => true,
        }
    }

    pub fn format_thousand_separator(&self, amount: u64) -> Result<String, FromUtf8Error> {
        let components: Result<Vec<String>, FromUtf8Error> = amount
            .to_string()
            .as_bytes()
            .rchunks(3)
            .rev()
            .map(|chunk| Ok(String::from_utf8(chunk.to_vec())?))
            .collect();

        Ok(components?.join("."))
    }
}
