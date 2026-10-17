//! Today's date without a date library. Dates are ISO 8601 strings (`YYYY-MM-DD`).

#[cfg(target_arch = "wasm32")]
pub fn today() -> String {
    let d = js_sys::Date::new_0();
    format!(
        "{:04}-{:02}-{:02}",
        d.get_full_year(),
        d.get_month() + 1,
        d.get_date()
    )
}

/// UTC date on native platforms.
#[cfg(not(target_arch = "wasm32"))]
pub fn today() -> String {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs());
    let (y, m, d) = civil_from_days((secs / 86_400) as i64);
    format!("{y:04}-{m:02}-{d:02}")
}

/// Days since 1970-01-01 to a calendar date (Howard Hinnant's algorithm).
#[cfg_attr(target_arch = "wasm32", allow(dead_code))]
fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (yoe + era * 400 + i64::from(m <= 2), m, d)
}

/// `2026-10-17` to `17.10.2026`.
pub fn display(iso: &str) -> String {
    let parts: Vec<&str> = iso.split('-').collect();
    match parts.as_slice() {
        [y, m, d] => format!("{d}.{m}.{y}"),
        _ => iso.to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn civil_dates() {
        assert_eq!(civil_from_days(0), (1970, 1, 1));
        assert_eq!(civil_from_days(19_723), (2024, 1, 1));
        assert_eq!(civil_from_days(20_743), (2026, 10, 17));
    }

    #[test]
    fn display_format() {
        assert_eq!(display("2026-10-17"), "17.10.2026");
        assert_eq!(display("x"), "x");
    }
}
