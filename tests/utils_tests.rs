use rev_report::utils::utils::format_currency;

#[test]
fn test_format_currency_usd_positive() {
    let result = format_currency(1234.56, "USD");
    assert_eq!(result, "$1,234.56");
}

#[test]
fn test_format_currency_usd_negative() {
    let result = format_currency(-1234.56, "USD");
    assert_eq!(result, "-$1,234.56");
}

#[test]
fn test_format_currency_usd_large_number() {
    let result = format_currency(1234567.89, "USD");
    assert_eq!(result, "$1,234,567.89");
}

#[test]
fn test_format_currency_usd_small_number() {
    let result = format_currency(100.00, "USD");
    assert_eq!(result, "$100.00");
}

#[test]
fn test_format_currency_usd_zero() {
    let result = format_currency(0.0, "USD");
    assert_eq!(result, "$0.00");
}

#[test]
fn test_format_currency_aud() {
    let result = format_currency(1234.56, "AUD");
    assert_eq!(result, "A$1,234.56");
}

#[test]
fn test_format_currency_aud_lowercase() {
    let result = format_currency(1234.56, "aud");
    assert_eq!(result, "A$1,234.56");
}

#[test]
fn test_format_currency_eur() {
    let result = format_currency(1234.56, "EUR");
    assert_eq!(result, "€1,234.56");
}

#[test]
fn test_format_currency_gbp() {
    let result = format_currency(1234.56, "GBP");
    assert_eq!(result, "£1,234.56");
}

#[test]
fn test_format_currency_cad() {
    let result = format_currency(1234.56, "CAD");
    assert_eq!(result, "C$1,234.56");
}

#[test]
fn test_format_currency_unknown_currency() {
    let result = format_currency(1234.56, "XYZ");
    assert_eq!(result, "1,234.56");
}

#[test]
fn test_format_currency_one_decimal_place() {
    let result = format_currency(1234.5, "USD");
    assert_eq!(result, "$1,234.50");
}

#[test]
fn test_format_currency_no_decimal_place() {
    let result = format_currency(1234.0, "USD");
    assert_eq!(result, "$1,234.00");
}

#[test]
fn test_format_currency_fractional_cents() {
    let result = format_currency(1234.567, "USD");
    assert_eq!(result, "$1,234.57"); // Rounded to 2 decimal places
}

#[test]
fn test_format_currency_very_large_number() {
    let result = format_currency(1234567890.12, "USD");
    assert_eq!(result, "$1,234,567,890.12");
}

#[test]
fn test_format_currency_single_digit() {
    let result = format_currency(9.99, "USD");
    assert_eq!(result, "$9.99");
}

#[test]
fn test_format_currency_less_than_one() {
    let result = format_currency(0.99, "USD");
    assert_eq!(result, "$0.99");
}
