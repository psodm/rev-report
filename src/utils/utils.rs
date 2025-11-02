/// Formats a currency value with thousand separators and currency symbol
/// Returns a formatted string like "$1,234.56" or "A$1,234.56"
pub fn format_currency(value: f64, currency_code: &str) -> String {
    let sign = if value < 0.0 { "-" } else { "" };
    let abs_value = value.abs();

    // Format with 2 decimal places
    let formatted = format!("{:.2}", abs_value);
    let parts: Vec<&str> = formatted.split('.').collect();
    let integer_part = parts[0];
    let decimal_part = parts.get(1).unwrap_or(&"00");

    // Add thousand separators
    let mut integer_with_commas = String::new();
    let chars: Vec<char> = integer_part.chars().rev().collect();
    for (i, ch) in chars.iter().enumerate() {
        if i > 0 && i % 3 == 0 {
            integer_with_commas.push(',');
        }
        integer_with_commas.push(*ch);
    }
    let integer_with_commas: String = integer_with_commas.chars().rev().collect();

    // Get currency symbol
    let symbol = match currency_code.to_uppercase().as_str() {
        "USD" => "$",
        "AUD" => "A$",
        "EUR" => "€",
        "GBP" => "£",
        "CAD" => "C$",
        _ => "", // No symbol for unknown currencies
    };

    format!("{}{}{}.{}", sign, symbol, integer_with_commas, decimal_part)
}

