//! Number formatting for German (decimal comma, thin grouping not used).

use healthyteller_core::Quantity;

/// Rounds to a sensible precision: whole numbers from 100, one decimal from 10, else two.
pub fn number(value: f64) -> String {
    let decimals = if value.abs() >= 100.0 {
        0
    } else if value.abs() >= 10.0 {
        1
    } else {
        2
    };
    let mut s = format!("{value:.decimals$}");
    if s.contains('.') {
        s = s.trim_end_matches('0').trim_end_matches('.').to_owned();
    }
    if s == "-0" {
        s = "0".to_owned();
    }
    s.replace('.', ",")
}

pub fn quantity(q: Quantity) -> String {
    format!("{} {}", number(q.amount), q.unit.symbol())
}

pub fn percent(fraction: f64) -> String {
    format!("{} %", number((fraction * 100.0).round()))
}

/// Parses user input with decimal comma or point; empty input is `None`.
pub fn parse_number(input: &str) -> Option<f64> {
    let s = input.trim().replace(',', ".");
    if s.is_empty() {
        return None;
    }
    s.parse::<f64>().ok().filter(|v| v.is_finite() && *v >= 0.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use healthyteller_core::Unit;

    #[test]
    fn german_numbers() {
        assert_eq!(number(1.5), "1,5");
        assert_eq!(number(0.126), "0,13");
        assert_eq!(number(12.34), "12,3");
        assert_eq!(number(150.0), "150");
        assert_eq!(number(2.0), "2");
        assert_eq!(number(-0.0001), "0");
        assert_eq!(quantity(Quantity::new(6.4, Unit::Gram)), "6,4 g");
        assert_eq!(percent(0.256), "26 %");
    }

    #[test]
    fn parses_input() {
        assert_eq!(parse_number("1,5"), Some(1.5));
        assert_eq!(parse_number(" 2.25 "), Some(2.25));
        assert_eq!(parse_number(""), None);
        assert_eq!(parse_number("-3"), None);
        assert_eq!(parse_number("abc"), None);
    }
}
