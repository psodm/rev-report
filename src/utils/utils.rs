use chrono::NaiveDate;

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

/// Parses a date string in dd/mm/yyyy format to NaiveDate
/// Returns None if the date string is empty or cannot be parsed
pub fn parse_date_from_dd_mm_yyyy(date_str: &str) -> Option<NaiveDate> {
    if date_str.is_empty() {
        return None;
    }

    let parts: Vec<&str> = date_str.split('/').collect();
    if parts.len() != 3 {
        return None;
    }

    let day = parts[0].parse::<u32>().ok()?;
    let month = parts[1].parse::<u32>().ok()?;
    let year = parts[2].parse::<i32>().ok()?;

    NaiveDate::from_ymd_opt(year, month, day)
}

/// Safely truncates a string to a maximum number of characters (not bytes)
/// This handles multi-byte UTF-8 characters correctly
pub fn truncate_string(s: &str, max_chars: usize) -> &str {
    if s.chars().count() <= max_chars {
        s
    } else {
        s.char_indices()
            .nth(max_chars)
            .map(|(idx, _)| &s[..idx])
            .unwrap_or(s)
    }
}

