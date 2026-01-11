pub mod constants;

use std::num::IntErrorKind;

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
                // @todo: crashed when input > 18 and < 20. ex: 9999999999999999999
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
        if income == 0 {
            return (0.0, "Tyduck kena pajak");
        }

        for bracket in TAX_BRACKETS {
            if income <= bracket.limit {
                return (bracket.rate, bracket.description);
            }
        }

        (0.0, "Error: error ape ni?")
    }

    pub fn format_thousand_separator(&self, amount: u64) -> String {
        let s = amount.to_string();
        let len = s.len();

        s.chars()
            .enumerate()
            .flat_map(|(i, ch)| {
                // Add separator after every 3 digits from the right
                if i > 0 && (len - i).is_multiple_of(3) {
                    vec!['.', ch]
                } else {
                    vec![ch]
                }
            })
            .collect()
    }
}
