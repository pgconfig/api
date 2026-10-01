//! The parsers of the v1 inputs. They accept what Go's `strconv` and the old
//! byte parser accepted, and their errors read the same, because REST v1
//! returns those messages to the caller.

use std::fmt;
use std::num::IntErrorKind;

use crate::bytes::{GB, KB, MB, TB};

/// A number Go's `strconv` would reject, with the message it would give.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseNumberError {
    function: &'static str,
    input: String,
    out_of_range: bool,
}

impl fmt::Display for ParseNumberError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let problem = if self.out_of_range {
            "value out of range"
        } else {
            "invalid syntax"
        };
        write!(
            f,
            "strconv.{}: parsing {:?}: {problem}",
            self.function, self.input
        )
    }
}

impl std::error::Error for ParseNumberError {}

/// Parses a PostgreSQL version the v1 way, as a `float32`. `17.10` becomes
/// `17.1`, and any number is accepted.
pub fn parse_pg_version(text: &str) -> Result<f32, ParseNumberError> {
    let error = |out_of_range| ParseNumberError {
        function: "ParseFloat",
        input: text.to_string(),
        out_of_range,
    };
    let value: f32 = text.parse().map_err(|_| error(false))?;
    let names_infinity = text
        .trim_start_matches(['+', '-'])
        .to_ascii_lowercase()
        .starts_with("inf");
    if value.is_infinite() && !names_infinity {
        return Err(error(true));
    }
    Ok(value)
}

/// Parses an integer the way Go's `strconv.Atoi` did.
pub fn parse_int(text: &str) -> Result<i64, ParseNumberError> {
    text.parse::<i64>().map_err(|err| ParseNumberError {
        function: "Atoi",
        input: text.to_string(),
        out_of_range: matches!(
            err.kind(),
            IntErrorKind::PosOverflow | IntErrorKind::NegOverflow
        ),
    })
}

/// Parses an amount of memory the v1 way: the leading digits, then a unit of
/// `kb`, `mb`, `gb`, or `tb` in any case. Any other suffix means bytes, and a
/// text that does not start with a digit is zero. Nothing is an error.
pub fn parse_bytes(text: &str) -> i64 {
    let digits = text.bytes().take_while(u8::is_ascii_digit).count();
    let (number, unit) = text.split_at(digits);
    let Ok(amount) = number.parse::<f64>() else {
        return 0;
    };
    let scale = match unit.trim().to_lowercase().as_str() {
        "kb" => KB,
        "mb" => MB,
        "gb" => GB,
        "tb" => TB,
        _ => 1,
    };
    (amount as i64).wrapping_mul(scale)
}

/// The key of a release in the documentation, the way v1 derived it from the
/// float: one decimal below 10, none from 10 on. `13.5` rounds to `14`.
pub fn format_version(pg_version: f32) -> String {
    if pg_version < 10.0 {
        format!("{pg_version:.1}")
    } else {
        format!("{pg_version:.0}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_version_is_a_float() {
        assert_eq!(parse_pg_version("17.10"), Ok(17.1));
        assert_eq!(parse_pg_version("9.60"), Ok(9.6));
        assert_eq!(parse_pg_version("1e1"), Ok(10.0));
    }

    #[test]
    fn a_bad_version_reads_like_a_strconv_error() {
        assert_eq!(
            parse_pg_version("abc").unwrap_err().to_string(),
            "strconv.ParseFloat: parsing \"abc\": invalid syntax"
        );
        assert_eq!(
            parse_pg_version("1e50").unwrap_err().to_string(),
            "strconv.ParseFloat: parsing \"1e50\": value out of range"
        );
    }

    #[test]
    fn a_bad_integer_reads_like_a_strconv_error() {
        assert_eq!(
            parse_int("1.5").unwrap_err().to_string(),
            "strconv.Atoi: parsing \"1.5\": invalid syntax"
        );
        assert_eq!(
            parse_int("99999999999999999999").unwrap_err().to_string(),
            "strconv.Atoi: parsing \"99999999999999999999\": value out of range"
        );
    }

    #[test]
    fn bytes_are_parsed_permissively() {
        for (text, bytes) in [
            ("2GB", 2 * GB),
            ("2gb", 2 * GB),
            ("2 GB", 2 * GB),
            ("100kb", 100 * KB),
            ("2048", 2048),
            ("2G", 2),
            ("1.5GB", 1),
            ("abc", 0),
            ("", 0),
            (" 2GB", 0),
        ] {
            assert_eq!(parse_bytes(text), bytes, "{text:?}");
        }
    }

    #[test]
    fn the_documentation_key_rounds_the_float() {
        assert_eq!(format_version(9.6), "9.6");
        assert_eq!(format_version(18.0), "18");
        assert_eq!(format_version(13.5), "14");
        assert_eq!(format_version(12.5), "12");
    }
}
