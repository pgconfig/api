//! Byte quantities, in the binary units PostgreSQL uses.

use std::fmt;

use serde::{Serialize, Serializer};

pub(crate) const KB: i64 = 1 << 10;
pub(crate) const MB: i64 = 1 << 20;
pub(crate) const GB: i64 = 1 << 30;
pub(crate) const TB: i64 = 1 << 40;

const UNITS: [(&str, i64); 5] = [("TB", TB), ("GB", GB), ("MB", MB), ("KB", KB), ("B", 1)];

/// A positive amount of memory.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bytes(i64);

/// Why a text is not a valid amount of memory.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BytesError(String);

impl fmt::Display for BytesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for BytesError {}

impl Bytes {
    /// Parses a positive integer followed by `B`, `KB`, `MB`, `GB`, or `TB`,
    /// in any case. A missing unit, a decimal, and zero are errors: a bare
    /// number is never guessed to be in some unit.
    pub fn parse(text: &str) -> Result<Self, BytesError> {
        let invalid = |problem: &str| {
            BytesError(format!(
                "{text:?} {problem}. Use a positive integer followed by B, KB, MB, GB, or TB, such as 16GB or 1536MB."
            ))
        };
        let trimmed = text.trim();
        let digits = trimmed.bytes().take_while(u8::is_ascii_digit).count();
        let (number, unit) = trimmed.split_at(digits);
        let unit = unit.trim_start();
        if number.is_empty() {
            return Err(invalid("does not start with a positive integer"));
        }
        if unit.is_empty() {
            return Err(invalid("has no unit"));
        }
        if unit.starts_with(['.', ',']) {
            return Err(invalid("has a decimal part"));
        }
        let Some((_, scale)) = UNITS
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case(unit))
        else {
            return Err(invalid("has an unknown unit"));
        };
        let amount = number
            .parse::<i64>()
            .ok()
            .and_then(|amount| amount.checked_mul(*scale))
            .ok_or_else(|| invalid("is too large"))?;
        if amount == 0 {
            return Err(invalid("is zero"));
        }
        Ok(Self(amount))
    }

    /// The amount in bytes.
    pub fn get(self) -> i64 {
        self.0
    }
}

/// Writes the amount without loss, in the largest unit that divides it:
/// `1536MB`, not `1.5GB` or `2GB`.
impl fmt::Display for Bytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (unit, scale) = UNITS
            .iter()
            .find(|(_, scale)| self.0 % scale == 0)
            .copied()
            .unwrap_or(("B", 1));
        write!(f, "{}{unit}", self.0 / scale)
    }
}

impl Serialize for Bytes {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

/// Formats a computed size the way pgconfig always has: rounded to the nearest
/// whole number of the largest unit that keeps the number below 1024. Zero and
/// negative values, such as the `-1` of `wal_buffers`, print as plain numbers.
/// Sizes of 1024TB and above print as an empty string, as they did in Go.
pub(crate) fn rounded(bytes: i64) -> String {
    let scaled = |unit: i64| (bytes as f64 / unit as f64).round();
    if bytes <= 0 {
        format!("{:.0}", scaled(1))
    } else if bytes < KB {
        format!("{:.0}B", scaled(1))
    } else if bytes < MB {
        format!("{:.0}kB", scaled(KB))
    } else if bytes < GB {
        format!("{:.0}MB", scaled(MB))
    } else if bytes < TB {
        format!("{:.0}GB", scaled(GB))
    } else if bytes < 1024 * TB {
        format!("{:.0}TB", scaled(TB))
    } else {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_every_unit_in_any_case() {
        for (text, bytes) in [
            ("512B", 512),
            ("4kb", 4 * KB),
            ("1536MB", 1536 * MB),
            ("16gb", 16 * GB),
            ("2Tb", 2 * TB),
            (" 8 GB ", 8 * GB),
        ] {
            assert_eq!(Bytes::parse(text).map(Bytes::get), Ok(bytes), "{text}");
        }
    }

    #[test]
    fn rejects_what_would_need_a_guess() {
        for text in [
            "16",
            "1.5GB",
            "1,5GB",
            "0GB",
            "-4GB",
            "GB",
            "",
            "16G",
            "16 gigs",
            "99999999999TB",
        ] {
            assert!(Bytes::parse(text).is_err(), "{text}");
        }
    }

    #[test]
    fn an_error_names_the_value_and_the_accepted_form() {
        let error = Bytes::parse("16").unwrap_err().to_string();

        assert_eq!(
            error,
            "\"16\" has no unit. Use a positive integer followed by B, KB, MB, GB, or TB, such as 16GB or 1536MB."
        );
    }

    #[test]
    fn displays_without_loss() {
        for (text, shown) in [
            ("1536MB", "1536MB"),
            ("16384MB", "16GB"),
            ("1024B", "1KB"),
            ("3b", "3B"),
        ] {
            assert_eq!(Bytes::parse(text).unwrap().to_string(), shown);
        }
    }

    #[test]
    fn rounds_computed_sizes_to_the_largest_unit() {
        for (bytes, shown) in [
            (-1, "-1"),
            (0, "0"),
            (1023, "1023B"),
            (KB, "1kB"),
            (5 * MB + 400 * KB, "5MB"),
            (1023 * MB + 700 * KB, "1024MB"),
            (GB, "1GB"),
            (768 * TB, "768TB"),
            (1024 * TB, ""),
        ] {
            assert_eq!(rounded(bytes), shown, "{bytes}");
        }
    }
}
