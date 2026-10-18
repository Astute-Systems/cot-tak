//! CoT timestamp handling (`2025-01-31T12:00:00.000Z`).

use crate::error::{Error, Result};
use time::format_description::well_known::Rfc3339;
use time::macros::format_description;
use time::{Duration, OffsetDateTime};

/// Parses an RFC 3339 / CoT timestamp, normalising to UTC.
pub fn parse(s: &str) -> Result<OffsetDateTime> {
    OffsetDateTime::parse(s.trim(), &Rfc3339)
        .map(|t| t.to_offset(time::UtcOffset::UTC))
        .map_err(|_| Error::InvalidTime(s.to_owned()))
}

/// Formats a timestamp in the canonical CoT form: UTC, millisecond precision.
pub fn format(t: OffsetDateTime) -> Result<String> {
    let fmt = format_description!(
        "[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:3]Z"
    );
    t.to_offset(time::UtcOffset::UTC)
        .format(&fmt)
        .map_err(|_| Error::InvalidTime(format!("{t:?}")))
}

/// Current UTC time truncated to whole milliseconds.
pub fn now() -> OffsetDateTime {
    truncate_ms(OffsetDateTime::now_utc())
}

/// Truncates sub-millisecond precision (the CoT wire format has none).
pub fn truncate_ms(t: OffsetDateTime) -> OffsetDateTime {
    let ns = t.nanosecond() % 1_000_000;
    t - Duration::nanoseconds(i64::from(ns))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip() {
        let t = parse("2025-01-31T12:34:56.789Z").unwrap();
        assert_eq!(format(t).unwrap(), "2025-01-31T12:34:56.789Z");
    }

    #[test]
    fn offsets_normalise_to_utc() {
        let t = parse("2025-01-31T22:34:56+10:00").unwrap();
        assert_eq!(format(t).unwrap(), "2025-01-31T12:34:56.000Z");
    }

    #[test]
    fn rejects_garbage() {
        assert!(parse("yesterday").is_err());
        assert!(parse("").is_err());
    }

    #[test]
    fn truncates() {
        let t = parse("2025-01-31T12:34:56.789123456Z").unwrap();
        assert_eq!(truncate_ms(t).nanosecond(), 789_000_000);
    }
}
