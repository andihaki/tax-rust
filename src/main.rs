use std::io;

/// Rust program to calculate a simplified, single-tier Income Tax (PPH 21)
/// based on Monthly Taxable Income (Penghasilan Kena Pajak - PKP).
///
/// Note: The tax rates provided are simplified for this example and represent the
/// official PPH 21 rates applied to PKP.
///
/// Tiers (Monthly PKP):
/// 1. IDR 0 – 50.000.000: 5%
/// 2. IDR 50.000.001 – 250.000.000: 15%
/// 3. IDR 250.000.001 – 500.000.000: 25%
/// 4. IDR 500.000.001 – 5.000.000.000: 30%
/// 5. > IDR 5.000.000.000: 35%

// --- CONSTANTS ---

// Define the upper limits (in IDR) for each tax bracket
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

/// Calculates the total tax using a single-tier lookup (simplified per request).
///
/// The entire income is taxed at the rate of the highest bracket it reaches.
///
/// # Arguments
/// * `montly_income` - The annual PKP (Penghasilan Kena Pajak) in IDR.
///
/// # Returns
/// The total annual tax owed as an f64.
fn calculate_pph21_tax(montly_income: u64) -> f64 {
    let income = montly_income * 12;
    let rate: f64;
    let bracket_name: &str;

    // Determine the highest applicable bracket using a single-tier lookup
    if income == 0 {
        rate = 0.0;
        bracket_name = "N/A";
    } else if income <= BRACKET_1_LIMIT {
        rate = RATE_1;
        bracket_name = "Bracket 1 (IDR 0 - 50.000.000)";
    } else if income <= BRACKET_2_LIMIT {
        rate = RATE_2;
        bracket_name = "Bracket 2 (IDR 50.000.001 - 250.000.000)";
    } else if income <= BRACKET_3_LIMIT {
        rate = RATE_3;
        bracket_name = "Bracket 3 (IDR 250.000.001 - 500.000.000)";
    } else if income <= BRACKET_4_LIMIT {
        rate = RATE_4;
        bracket_name = "Bracket 4 (IDR 500.000.001 - 5.000.000.000)";
    } else {
        // > 5B
        rate = RATE_5;
        bracket_name = "Bracket 5 (> IDR 5.000.000.000)";
    }

    let total_tax: f64 = (income as f64) * rate;

    println!(
        "\n--- Calculation for Monthly PKP: IDR {:} ---",
        format_income(income)
    );

    let percentage = format!("{}%", rate * 100.0);
    // Print only the single applicable part of the calculation
    if income > 0 {
        println!(
            "Income of IDR {} falls into {} and is entirely taxed at {}.",
            format_income(income),
            bracket_name,
            percentage
        );
    } else {
        println!("No taxable income, no tax calculated.");
    }

    println!("Total PKP Tax Due: IDR {:.2}", total_tax);

    total_tax
}

/// Helper function to format the u64 income into a string with thousands separators.
fn format_income(amount: u64) -> String {
    // This is a basic way to add thousands separators for display purposes
    amount
        .to_string()
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(|chunk| String::from_utf8(chunk.to_vec()).unwrap())
        .collect::<Vec<String>>()
        .join(".")
}

fn main() {
    println!("=== PPH 21 Progressive Tax Calculator ===");
    println!("(Calculates Monthly Tax on Taxable Income - PKP)");

    let mut input = String::new();

    // Prompt user for input
    println!("\nEnter your Monthly Taxable Income (PKP) in IDR (e.g., 400000000):");

    // Read input from stdin
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            // Trim whitespace and parse the input string into a u64
            match input.trim().parse::<u64>() {
                Ok(income) => {
                    // Calculate and print the tax
                    calculate_pph21_tax(income);
                }
                Err(_) => {
                    // Handle non-numeric or overflow errors
                    eprintln!(
                        "Error: Invalid input. Please enter a whole number without any commas, periods, or other non-digit characters."
                    );
                }
            }
        }
        Err(error) => eprintln!("Error reading input: {}", error),
    }
}
