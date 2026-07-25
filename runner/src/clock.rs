//! Timestamps without a date crate.
//!
//! The runner needs exactly one thing from a clock: an ISO-8601 UTC string for
//! the progress file and the report. Pulling in `chrono`/`time` for that would
//! add dependencies to a binary that must stay small and boring.

use std::time::{SystemTime, UNIX_EPOCH};

/// Current time as `YYYY-MM-DDTHH:MM:SSZ` (UTC).
pub fn now_iso8601() -> String {
    let secs = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(d) => d.as_secs() as i64,
        // Clock before 1970 (misconfigured VM). Fall back to the epoch instead
        // of panicking — a wrong timestamp must never stop a learner.
        Err(_) => 0,
    };
    format_epoch_utc(secs)
}

/// Format Unix seconds as `YYYY-MM-DDTHH:MM:SSZ`.
pub fn format_epoch_utc(secs: i64) -> String {
    let days = secs.div_euclid(86_400);
    let rest = secs.rem_euclid(86_400);
    let (year, month, day) = civil_from_days(days);
    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        year,
        month,
        day,
        rest / 3600,
        (rest % 3600) / 60,
        rest % 60
    )
}

/// Howard Hinnant's `civil_from_days`: days since 1970-01-01 -> (y, m, d).
fn civil_from_days(days: i64) -> (i64, u32, u32) {
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64; // [0, 146096]
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365; // [0, 399]
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); // [0, 365]
    let mp = (5 * doy + 2) / 153; // [0, 11]
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32; // [1, 31]
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32; // [1, 12]
    (if m <= 2 { y + 1 } else { y }, m, d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn epoch_start() {
        assert_eq!(format_epoch_utc(0), "1970-01-01T00:00:00Z");
    }

    #[test]
    fn known_timestamps() {
        assert_eq!(format_epoch_utc(1_753_444_800), "2025-07-25T12:00:00Z");
        assert_eq!(format_epoch_utc(1_784_980_799), "2026-07-25T11:59:59Z");
        // leap day
        assert_eq!(format_epoch_utc(1_709_208_000), "2024-02-29T12:00:00Z");
    }

    #[test]
    fn before_epoch_does_not_panic() {
        assert_eq!(format_epoch_utc(-1), "1969-12-31T23:59:59Z");
    }

    #[test]
    fn now_has_expected_shape() {
        let now = now_iso8601();
        assert_eq!(now.len(), 20, "unexpected timestamp: {now}");
        assert!(now.ends_with('Z'));
    }
}
