//! PostgreSQL versions.

use std::fmt;

use serde::{Serialize, Serializer};

/// A PostgreSQL major version: two number groups before PostgreSQL 10
/// (`9.6`), one from 10 on (`18`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PgMajor {
    first: u32,
    /// The second group of a 9.x release. Zero from PostgreSQL 10 on.
    second: u32,
}

impl PgMajor {
    /// The oldest and the newest supported 9.x series.
    const NINE: std::ops::RangeInclusive<u32> = 1..=6;
    /// The supported series from PostgreSQL 10 on.
    const MODERN: std::ops::RangeInclusive<u32> = 10..=18;

    /// Every supported major version, oldest first.
    pub fn supported() -> impl Iterator<Item = PgMajor> {
        Self::NINE
            .map(|second| PgMajor { first: 9, second })
            .chain(Self::MODERN.map(|first| PgMajor { first, second: 0 }))
    }

    /// The position of this release on the float scale the rules compare
    /// versions on: 9.6 for PostgreSQL 9.6, 18.0 for PostgreSQL 18. REST v1
    /// defined that scale, and for a supported major it is exact.
    pub(crate) fn rule_scale(self) -> f32 {
        self.to_string()
            .parse()
            .expect("a major version is a decimal number")
    }
}

impl fmt::Display for PgMajor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.first < 10 {
            write!(f, "{}.{}", self.first, self.second)
        } else {
            write!(f, "{}", self.first)
        }
    }
}

/// A PostgreSQL version as supplied, such as `9.6.24`, `17.10`, or `18`. The
/// text is kept as it is, so `17.10` never turns into `17.1`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PgVersion {
    text: String,
    major: PgMajor,
}

/// Why a text is not a supported PostgreSQL version.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PgVersionError(String);

impl fmt::Display for PgVersionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for PgVersionError {}

impl PgVersion {
    /// Parses a dotted numeric version and checks that its major version is
    /// supported. The minor release is not checked against the releases that
    /// were published.
    pub fn parse(text: &str) -> Result<Self, PgVersionError> {
        let text = text.trim();
        let malformed = || {
            PgVersionError(format!(
                "{text:?} is not a PostgreSQL version. Use dotted numbers such as 9.6.24, 17.10, or 18."
            ))
        };
        let groups: Vec<u32> = text
            .split('.')
            .map(|group| {
                let numeric = !group.is_empty() && group.bytes().all(|b| b.is_ascii_digit());
                numeric.then(|| group.parse().ok()).flatten()
            })
            .collect::<Option<_>>()
            .ok_or_else(malformed)?;

        let major = match groups[..] {
            [first, second] | [first, second, _] if first < 10 => PgMajor { first, second },
            [first] | [first, _] if first >= 10 => PgMajor { first, second: 0 },
            _ => return Err(malformed()),
        };
        let supported = match major.first {
            9 => PgMajor::NINE.contains(&major.second),
            first => PgMajor::MODERN.contains(&first),
        };
        if !supported {
            return Err(PgVersionError(format!(
                "PostgreSQL {major} is not supported. Supported major versions are 9.1 to 9.6 and 10 to 18."
            )));
        }
        Ok(Self {
            text: text.to_string(),
            major,
        })
    }

    /// The version as supplied, without surrounding whitespace.
    pub fn as_str(&self) -> &str {
        &self.text
    }

    pub fn major(&self) -> PgMajor {
        self.major
    }
}

impl fmt::Display for PgVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.text)
    }
}

impl Serialize for PgVersion {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn major(text: &str) -> String {
        PgVersion::parse(text).unwrap().major().to_string()
    }

    #[test]
    fn derives_the_major_by_the_postgresql_versioning_policy() {
        assert_eq!(major("9.6"), "9.6");
        assert_eq!(major("9.6.24"), "9.6");
        assert_eq!(major("10"), "10");
        assert_eq!(major("17.10"), "17");
        assert_eq!(major("18.4"), "18");
    }

    #[test]
    fn keeps_the_supplied_text() {
        assert_eq!(PgVersion::parse(" 17.10 ").unwrap().as_str(), "17.10");
    }

    #[test]
    fn rejects_malformed_versions() {
        for text in [
            "v18.4", "18.x", "9..6", "", "9", "18.4.1", "9.6.24.1", "18.", ".18", "1e1", "-18",
        ] {
            let error = PgVersion::parse(text).unwrap_err().to_string();
            assert!(
                error.contains("is not a PostgreSQL version"),
                "{text}: {error}"
            );
        }
    }

    #[test]
    fn rejects_unsupported_major_versions() {
        for text in ["8.4", "9.0", "9.7", "19", "19.1", "7.4.30"] {
            let error = PgVersion::parse(text).unwrap_err().to_string();
            assert!(error.contains("is not supported"), "{text}: {error}");
        }
    }

    #[test]
    fn supports_fifteen_major_versions() {
        let supported: Vec<String> = PgMajor::supported()
            .map(|major| major.to_string())
            .collect();

        assert_eq!(
            supported,
            [
                "9.1", "9.2", "9.3", "9.4", "9.5", "9.6", "10", "11", "12", "13", "14", "15", "16",
                "17", "18"
            ]
        );
    }

    #[test]
    fn the_rule_scale_orders_majors_as_released() {
        let scale: Vec<f32> = PgMajor::supported().map(PgMajor::rule_scale).collect();

        assert!(scale.is_sorted());
        assert_eq!(scale[5], 9.6);
        assert_eq!(scale[14], 18.0);
    }
}
