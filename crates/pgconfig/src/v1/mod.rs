//! The v1 projection: the output of REST v1 and `pgconfigctl`, reproduced
//! exactly, quirks included.
//!
//! v1 is a public contract that installers call unattended, so its behavior is
//! frozen by the goldens in `tests/golden`. Its known defects stay here and
//! out of the rules:
//!
//! - The PostgreSQL version is a `float32`, so `17.10` is read as `17.1`, and
//!   it is never checked against the supported releases.
//! - The operating system is validated without regard to case, but only the
//!   exact text `windows` gets the Windows rules.
//! - An unrecognized drive type is accepted. It gets the HDD value of
//!   `effective_io_concurrency` and the SSD value of `random_page_cost`.
//! - `listen_addresses = '*'` is part of the output.
//!
//! New consumers should call [`crate::tune`] instead.

mod export;
mod parse;

use std::collections::BTreeMap;
use std::fmt;

use serde::Serialize;

use crate::docs;
use crate::request::Profile;
use crate::rules::{self, Disk, Facts, Value};

pub use export::{EXPORT_FORMATS, export};
pub use parse::{ParseNumberError, format_version, parse_bytes, parse_int, parse_pg_version};

/// The inputs of a v1 tuning call, after each interface parsed its arguments.
#[derive(Clone, Debug, PartialEq)]
pub struct Input {
    pub pg_version: f32,
    /// Bytes.
    pub total_ram: i64,
    pub total_cpu: i64,
    pub max_connections: i64,
    pub profile: Profile,
    pub os: String,
    pub arch: String,
    pub drive_type: String,
}

/// Why the rules reject an [`Input`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RuleError {
    InvalidArch,
    InvalidOs,
}

impl fmt::Display for RuleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            RuleError::InvalidArch => "Invalid Architecture",
            RuleError::InvalidOs => "Invalid OS",
        })
    }
}

impl std::error::Error for RuleError {}

/// A profile name v1 does not know.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnknownProfile;

impl fmt::Display for UnknownProfile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("must be one of [WEB OLTP DW MIXED DESKTOP]")
    }
}

impl std::error::Error for UnknownProfile {}

/// Parses a profile name the v1 way: any case, no surrounding whitespace.
pub fn parse_profile(text: &str) -> Result<Profile, UnknownProfile> {
    Profile::ALL
        .iter()
        .copied()
        .find(|profile| profile.as_str() == text.to_uppercase())
        .ok_or(UnknownProfile)
}

/// A group of parameters in the v1 output.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Category {
    #[serde(rename = "category")]
    pub name: &'static str,
    pub description: &'static str,
    /// `None` only in the placeholder v1 emits for an unknown log format,
    /// which serializes as `null`.
    pub parameters: Option<Vec<Parameter>>,
}

impl Category {
    pub(crate) fn parameters(&self) -> &[Parameter] {
        self.parameters.as_deref().unwrap_or_default()
    }
}

/// One parameter in the v1 output.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Parameter {
    pub name: &'static str,
    #[serde(rename = "config_value")]
    pub value: String,
    /// The Go type name v1 always exposed: `Byte`, `int`, `float32`,
    /// `string`, or `bool`.
    pub format: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation: Option<Documentation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<&'static str>,
}

/// What `show_doc=true` attaches to a parameter. Empty fields are left out.
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct Documentation {
    #[serde(rename = "name", skip_serializing_if = "str::is_empty")]
    pub title: &'static str,
    #[serde(skip_serializing_if = "str::is_empty")]
    pub short_desc: &'static str,
    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    pub details: &'static [&'static str],
    #[serde(skip_serializing_if = "str::is_empty")]
    pub url: &'static str,
    #[serde(skip_serializing_if = "str::is_empty")]
    pub conf_url: &'static str,
    #[serde(skip_serializing_if = "str::is_empty")]
    pub recomendations_conf: &'static str,
    #[serde(rename = "type", skip_serializing_if = "str::is_empty")]
    pub param_type: &'static str,
    #[serde(skip_serializing_if = "str::is_empty")]
    pub default_value: &'static str,
    #[serde(skip_serializing_if = "str::is_empty")]
    pub min_value: &'static str,
    #[serde(skip_serializing_if = "str::is_empty")]
    pub max_value: &'static str,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub recomendations: BTreeMap<&'static str, &'static str>,
    #[serde(rename = "abstract", skip_serializing_if = "str::is_empty")]
    pub abstract_text: &'static str,
}

/// Computes the v1 categories for an input. `pgbadger_log_format` adds the
/// pgbadger logging category and the category of that log format.
pub fn categories(
    input: &Input,
    pgbadger_log_format: Option<&str>,
) -> Result<Vec<Category>, RuleError> {
    // The architecture was the first rule to run, so its error wins.
    let thirty_two_bit = match input.arch.as_str() {
        "386" | "i686" => true,
        "amd64" | "x86-64" | "arm" | "arm64" => false,
        _ => return Err(RuleError::InvalidArch),
    };
    if !["windows", "linux", "unix", "darwin"].contains(&input.os.to_lowercase().as_str()) {
        return Err(RuleError::InvalidOs);
    }

    let computed = rules::compute(&Facts {
        total_ram: input.total_ram,
        total_cpu: input.total_cpu,
        max_connections: input.max_connections,
        profile: input.profile,
        disk: match input.drive_type.as_str() {
            "HDD" => Disk::Hdd,
            "SSD" => Disk::Ssd,
            "SAN" => Disk::San,
            _ => Disk::Unrecognized,
        },
        windows: input.os == "windows",
        thirty_two_bit,
        version: input.pg_version,
    });

    let mut categories: Vec<Category> = computed
        .groups()
        .into_iter()
        .filter(|group| !group.settings.is_empty())
        .map(|group| Category {
            name: group.id,
            description: group.description,
            parameters: Some(
                group
                    .settings
                    .into_iter()
                    .map(|(name, value)| {
                        let format = match value {
                            Value::Bytes(_) => "Byte",
                            Value::Int(_) => "int",
                            Value::Float(_) => "float32",
                            Value::Text(_) => "string",
                        };
                        parameter(name, &value.render(), format)
                    })
                    .collect(),
            ),
        })
        .collect();

    if let Some(log_format) = pgbadger_log_format {
        categories.push(pgbadger(input.pg_version));
        categories.push(log_options(log_format));
    }
    Ok(categories)
}

fn parameter(name: &'static str, value: &str, format: &'static str) -> Parameter {
    Parameter {
        name,
        value: value.to_string(),
        format,
        documentation: None,
        comment: None,
    }
}

fn pgbadger(pg_version: f32) -> Category {
    let mut parameters = vec![
        parameter("logging_collector", "on", "bool"),
        parameter("log_checkpoints", "on", "bool"),
        parameter("log_connections", "on", "bool"),
        parameter("log_disconnections", "on", "bool"),
        parameter("log_lock_waits", "on", "bool"),
        parameter("log_temp_files", "0", "int"),
        parameter("lc_messages", "C", "string"),
        parameter("log_min_duration_statement", "10s", "string"),
        parameter("log_autovacuum_min_duration", "0", "int"),
    ];
    parameters[7].comment = Some("Adjust the minimum time to collect the data");
    if pg_version >= 19.0 {
        parameters.push(parameter("log_autoanalyze_min_duration", "0", "int"));
    }
    Category {
        name: "log_config",
        description: "Logging configuration for pgbadger",
        parameters: Some(parameters),
    }
}

fn log_options(log_format: &str) -> Category {
    let text = |name, value| parameter(name, value, "string");
    let (name, description, parameters) = match log_format {
        "stderr" => (
            // The misspelling is part of the v1 output.
            "stder_config",
            "STDERR Configuration",
            vec![
                text("log_destination", "stderr"),
                text(
                    "log_line_prefix",
                    "%t [%p]: [%l-1] user=%u,db=%d,app=%a,client=%h ",
                ),
            ],
        ),
        "syslog" => (
            "syslog_config",
            "SYSLOG Configuration",
            vec![
                text("log_destination", "syslog"),
                text("log_line_prefix", "user=%u,db=%d,app=%a,client=%h "),
                text("syslog_facility", "LOCAL0"),
                text("syslog_ident", "postgres"),
            ],
        ),
        "csvlog" => (
            "csv_config",
            "CSV Configuration",
            vec![text("log_destination", "csvlog")],
        ),
        "jsonlog" => (
            "jsonlog_config",
            "JSON Log Configuration",
            vec![text("log_destination", "jsonlog")],
        ),
        // v1 never validated the log format. An unknown one adds this empty
        // category.
        _ => {
            return Category {
                name: "",
                description: "",
                parameters: None,
            };
        }
    };
    Category {
        name,
        description,
        parameters: Some(parameters),
    }
}

/// Attaches the documentation `show_doc=true` returns to every parameter.
pub fn add_documentation(categories: &mut [Category], pg_version: f32) {
    let release = format_version(pg_version);
    for category in categories {
        let category_name = category.name;
        for parameter in category.parameters.iter_mut().flatten() {
            let mut documentation = Documentation::default();
            if let Some(doc) = docs::param(&release, parameter.name) {
                documentation = Documentation {
                    title: doc.title,
                    short_desc: doc.short_desc,
                    details: doc.details,
                    url: doc.url,
                    conf_url: doc.conf_url,
                    recomendations_conf: doc.recomendations_conf,
                    param_type: doc.param_type,
                    default_value: doc.default_value,
                    min_value: doc.min_value,
                    max_value: doc.max_value,
                    ..documentation
                };
            }
            if let Some(rule) = docs::rule(category_name, parameter.name) {
                documentation.recomendations = rule.recomendations.iter().copied().collect();
                documentation.abstract_text = rule.abstract_text;
            }
            parameter.documentation = Some(documentation);
        }
    }
}
