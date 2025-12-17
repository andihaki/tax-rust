pub const MAX_INPUT_LENGTH: usize = 18;

pub const BRACKET_1_LIMIT: u64 = 50_000_000;
pub const BRACKET_2_LIMIT: u64 = 250_000_000;
pub const BRACKET_3_LIMIT: u64 = 500_000_000;
pub const BRACKET_4_LIMIT: u64 = 5_000_000_000;

pub const RATE_1: f64 = 0.05;
pub const RATE_2: f64 = 0.15;
pub const RATE_3: f64 = 0.25;
pub const RATE_4: f64 = 0.30;
pub const RATE_5: f64 = 0.35;

pub struct TaxBracket {
    // @todo: kenapa di dalem struct juga harus pub?
    pub limit: u64,
    pub rate: f64,
    pub description: &'static str,
}

pub const TAX_BRACKETS: [TaxBracket; 5] = [
    TaxBracket {
        limit: BRACKET_1_LIMIT,
        rate: RATE_1,
        description: "Golongan 1 (0 - 50 juta rupiah)",
    },
    TaxBracket {
        limit: BRACKET_2_LIMIT,
        rate: RATE_2,
        description: "Golongan 2 (50 - 250 juta rupiah)",
    },
    TaxBracket {
        limit: BRACKET_3_LIMIT,
        rate: RATE_3,
        description: "Golongan 3 (250 - 500 juta rupiah)",
    },
    TaxBracket {
        limit: BRACKET_4_LIMIT,
        rate: RATE_4,
        description: "Golongan Gajhi Nyuyok (500 juta - 5 miliar rupiah)",
    },
    TaxBracket {
        limit: u64::MAX,
        rate: RATE_5,
        description: "Golongan Sultan (> 5 miliar rupiah)",
    },
];
