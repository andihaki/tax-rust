pub struct TaxBreakdown {
    pub integer: u64,
    pub decimal: String,
}

impl TaxBreakdown {
    pub fn from(amount: f64) -> Self {
        let total_cents = (amount * 100.0).round() as u64;
        let integer = total_cents / 100;
        let decimal = format!("{:02}", total_cents % 100);
        Self { integer, decimal }
    }
}
