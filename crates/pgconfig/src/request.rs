//! The Tuning Request: the facts about a server that the recommendations are
//! computed from.

use std::fmt;
use std::num::NonZeroU32;

use serde::{Deserialize, Serialize, Serializer};

use crate::bytes::{Bytes, BytesError};
use crate::version::PgVersion;

/// Defines an enum whose variants each have one canonical name and a set of
/// accepted spellings, matched without regard to case.
macro_rules! named_enum {
    (
        $(#[$meta:meta])*
        $name:ident, $what:literal {
            $($(#[$variant_meta:meta])* $variant:ident => $canonical:literal $(| $alias:literal)*),+ $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum $name {
            $($(#[$variant_meta])* $variant),+
        }

        impl $name {
            /// Every value, in the order the documentation lists them.
            pub const ALL: &'static [$name] = &[$($name::$variant),+];

            /// The canonical name.
            pub fn as_str(self) -> &'static str {
                match self {
                    $($name::$variant => $canonical),+
                }
            }

            /// Parses a name or an accepted alias, in any case.
            pub fn parse(text: &str) -> Result<Self, String> {
                let trimmed = text.trim();
                $(
                    if [$canonical $(, $alias)*].iter().any(|name| name.eq_ignore_ascii_case(trimmed)) {
                        return Ok($name::$variant);
                    }
                )+
                let accepted: Vec<&str> = vec![$($canonical $(, $alias)*),+];
                Err(format!("{text:?} is not a known {}. Use one of: {}.", $what, accepted.join(", ")))
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(self.as_str())
            }
        }

        impl Serialize for $name {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.serialize_str(self.as_str())
            }
        }
    };
}

named_enum! {
    /// The workload the server runs.
    Profile, "profile" {
        /// General web applications.
        Web => "WEB",
        /// Transaction processing: many short writes.
        Oltp => "OLTP",
        /// Data warehouse: few large analytical queries.
        Dw => "DW",
        /// The database shares the machine with the application.
        Mixed => "MIXED",
        /// A development machine, where PostgreSQL should stay small.
        Desktop => "DESKTOP",
    }
}

named_enum! {
    /// The kind of storage under the data directory.
    DiskType, "disk type" {
        Ssd => "SSD",
        Hdd => "HDD",
        San => "SAN",
    }
}

named_enum! {
    /// The operating system PostgreSQL runs on.
    Os, "operating system" {
        Linux => "linux",
        Windows => "windows",
        Unix => "unix",
        Darwin => "darwin",
    }
}

named_enum! {
    /// The CPU architecture PostgreSQL is built for.
    Arch, "architecture" {
        /// 32-bit x86.
        X86 => "386" | "i686",
        Amd64 => "amd64" | "x86-64",
        Arm => "arm",
        Arm64 => "arm64",
    }
}

/// A Tuning Request with every supplied fact already validated. The optional
/// facts are `None` when the caller did not supply them; `tune` fills them in
/// and reports each one as a Tuning Assumption.
#[derive(Clone, Debug, PartialEq)]
pub struct TuningRequest {
    /// Memory dedicated to PostgreSQL.
    pub total_ram: Bytes,
    /// Logical CPUs, hyperthreads included.
    pub total_cpu: NonZeroU32,
    pub postgres_version: PgVersion,
    pub profile: Option<Profile>,
    pub disk_type: Option<DiskType>,
    pub os: Option<Os>,
    pub arch: Option<Arch>,
    pub max_connections: Option<NonZeroU32>,
}

/// A Tuning Request as it arrives from outside: untyped, with any field
/// possibly missing. `TuningRequest::try_from` validates it.
#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
pub struct RawTuningRequest {
    pub total_ram: Option<String>,
    pub total_cpu: Option<i64>,
    pub postgres_version: Option<String>,
    pub profile: Option<String>,
    pub disk_type: Option<String>,
    pub os: Option<String>,
    pub arch: Option<String>,
    pub max_connections: Option<i64>,
}

/// One reason a raw request cannot be used.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Problem {
    /// The request field the problem is about.
    pub field: &'static str,
    /// Whether the field was absent, as opposed to present and invalid.
    pub missing: bool,
    pub message: String,
}

/// Everything wrong with a raw request, so the caller can fix it in one go.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TuningError {
    pub problems: Vec<Problem>,
}

impl TuningError {
    /// The required fields that were absent.
    pub fn missing_fields(&self) -> Vec<&'static str> {
        self.problems
            .iter()
            .filter(|problem| problem.missing)
            .map(|problem| problem.field)
            .collect()
    }
}

impl fmt::Display for TuningError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let messages: Vec<&str> = self.problems.iter().map(|p| p.message.as_str()).collect();
        write!(f, "Invalid tuning request. {}", messages.join(" "))
    }
}

impl std::error::Error for TuningError {}

impl TryFrom<RawTuningRequest> for TuningRequest {
    type Error = TuningError;

    /// Validates every field and reports all the problems, in field order.
    fn try_from(raw: RawTuningRequest) -> Result<Self, TuningError> {
        let mut problems = Vec::new();
        let text = |result: Result<_, _>| result.map_err(|error: BytesError| error.to_string());

        let total_ram = required(
            &mut problems,
            "total_ram",
            "the memory dedicated to PostgreSQL, such as 16GB",
            raw.total_ram.as_deref().map(|ram| text(Bytes::parse(ram))),
        );
        let total_cpu = required(
            &mut problems,
            "total_cpu",
            "the number of logical CPUs, such as 8",
            raw.total_cpu.map(positive),
        );
        let postgres_version = required(
            &mut problems,
            "postgres_version",
            "the PostgreSQL version, such as 18.4",
            raw.postgres_version
                .as_deref()
                .map(|version| PgVersion::parse(version).map_err(|error| error.to_string())),
        );
        let profile = optional(
            &mut problems,
            "profile",
            raw.profile.as_deref().map(Profile::parse),
        );
        let disk_type = optional(
            &mut problems,
            "disk_type",
            raw.disk_type.as_deref().map(DiskType::parse),
        );
        let os = optional(&mut problems, "os", raw.os.as_deref().map(Os::parse));
        let arch = optional(&mut problems, "arch", raw.arch.as_deref().map(Arch::parse));
        let max_connections = optional(
            &mut problems,
            "max_connections",
            raw.max_connections.map(positive),
        );

        match (total_ram, total_cpu, postgres_version) {
            (Some(total_ram), Some(total_cpu), Some(postgres_version)) if problems.is_empty() => {
                Ok(TuningRequest {
                    total_ram,
                    total_cpu,
                    postgres_version,
                    profile,
                    disk_type,
                    os,
                    arch,
                    max_connections,
                })
            }
            _ => Err(TuningError { problems }),
        }
    }
}

/// The value of a required field, recording a problem when it is absent or
/// invalid.
fn required<T>(
    problems: &mut Vec<Problem>,
    field: &'static str,
    hint: &str,
    parsed: Option<Result<T, String>>,
) -> Option<T> {
    if parsed.is_none() {
        let message = format!("{field} is required: {hint}.");
        problems.push(Problem {
            field,
            missing: true,
            message,
        });
    }
    optional(problems, field, parsed)
}

/// The value of an optional field, recording a problem when it is invalid.
fn optional<T>(
    problems: &mut Vec<Problem>,
    field: &'static str,
    parsed: Option<Result<T, String>>,
) -> Option<T> {
    match parsed? {
        Ok(value) => Some(value),
        Err(message) => {
            let message = format!("{field}: {message}");
            problems.push(Problem {
                field,
                missing: false,
                message,
            });
            None
        }
    }
}

fn positive(count: i64) -> Result<NonZeroU32, String> {
    u32::try_from(count)
        .ok()
        .and_then(NonZeroU32::new)
        .ok_or_else(|| format!("{count} is not a positive integer."))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_parse_in_any_case_and_normalize() {
        assert_eq!(Profile::parse("oltp").unwrap().as_str(), "OLTP");
        assert_eq!(DiskType::parse(" san ").unwrap().as_str(), "SAN");
        assert_eq!(Os::parse("Windows").unwrap().as_str(), "windows");
        assert_eq!(Arch::parse("AMD64").unwrap().as_str(), "amd64");
    }

    #[test]
    fn architecture_aliases_normalize() {
        assert_eq!(Arch::parse("i686").unwrap(), Arch::X86);
        assert_eq!(Arch::parse("X86-64").unwrap(), Arch::Amd64);
    }

    #[test]
    fn an_unknown_name_lists_the_accepted_ones() {
        assert_eq!(
            DiskType::parse("nvme").unwrap_err(),
            "\"nvme\" is not a known disk type. Use one of: SSD, HDD, SAN."
        );
        assert_eq!(
            Arch::parse("ppc64").unwrap_err(),
            "\"ppc64\" is not a known architecture. Use one of: 386, i686, amd64, x86-64, arm, arm64."
        );
    }
}
