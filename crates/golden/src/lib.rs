//! Golden files recorded from the pgconfig binaries, and the harness that
//! records and replays them.
//!
//! The files live in `tests/golden` at the repository root. `record` writes
//! them from a server binary and a CLI binary. `check` replays every stored
//! request against the binaries and reports what differs. Both take the
//! binaries as paths, so the same goldens hold the Go binaries they were
//! recorded from and the Rust binaries that replace them.

mod cases;
mod http;
pub mod normalize;
mod record;
mod run;

use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use serde_json::Value;

pub use cases::{CliGroup, PROFILES, RestCase, RestGroup, VERSIONS, cli_groups, rest_groups};
pub use http::HOST;
pub use record::{CliRecord, RestRecord, canonical, read_records};

/// Response headers a REST record pins, when its group pins headers.
pub const PINNED_HEADERS: [&str; 7] = [
    "access-control-allow-methods",
    "access-control-allow-origin",
    "allow",
    "cache-control",
    "content-type",
    "location",
    "vary",
];

/// `tests/golden` at the repository root.
pub fn golden_dir() -> PathBuf {
    repo_root().join("tests/golden")
}

/// The repository root, found from this crate's manifest.
pub fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repository root")
}

/// The stored REST records, one entry per golden file.
pub fn stored_rest(dir: &Path) -> io::Result<Vec<(String, Vec<RestRecord>)>> {
    stored(&dir.join("rest"))
}

/// The stored CLI records, one entry per golden file.
pub fn stored_cli(dir: &Path) -> io::Result<Vec<(String, Vec<CliRecord>)>> {
    stored(&dir.join("cli"))
}

fn stored<T: serde::de::DeserializeOwned>(dir: &Path) -> io::Result<Vec<(String, Vec<T>)>> {
    let mut files: Vec<PathBuf> = fs::read_dir(dir)?
        .collect::<io::Result<Vec<_>>>()?
        .into_iter()
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "jsonl"))
        .collect();
    files.sort();
    files
        .into_iter()
        .map(|path| {
            let name = path
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned();
            Ok((name, read_records(&path)?))
        })
        .collect()
}

/// Runs every REST case against `server_bin` and returns the records.
fn run_rest(server_bin: &Path) -> io::Result<Vec<(String, Vec<RestRecord>)>> {
    let server = run::Server::spawn(server_bin, &repo_root())?;
    let mut client = server.client();
    let mut groups = Vec::new();
    for group in rest_groups() {
        let mut records = Vec::with_capacity(group.cases.len());
        for case in &group.cases {
            let request_headers: BTreeMap<String, String> = case
                .request_headers
                .iter()
                .map(|(name, value)| (name.to_string(), value.to_string()))
                .collect();
            records.push(rest_exchange(
                &mut client,
                case.method,
                &case.path,
                request_headers,
                group.pin_headers,
            )?);
        }
        groups.push((group.name, records));
    }
    Ok(groups)
}

fn rest_exchange(
    client: &mut http::Client,
    method: &str,
    path: &str,
    request_headers: BTreeMap<String, String>,
    pin_headers: bool,
) -> io::Result<RestRecord> {
    let response = client.request(method, path, &request_headers)?;
    let headers = if pin_headers {
        response
            .headers
            .iter()
            .filter(|(name, _)| PINNED_HEADERS.contains(&name.as_str()))
            .map(|(name, value)| (name.clone(), value.clone()))
            .collect()
    } else {
        BTreeMap::new()
    };
    let is_json = response
        .headers
        .get("content-type")
        .is_some_and(|v| v.starts_with("application/json"));
    let body = String::from_utf8(response.body).map_err(io::Error::other)?;
    let (json, text) = if is_json && !body.is_empty() {
        let mut value: Value = serde_json::from_str(&body)
            .map_err(|err| io::Error::other(format!("{method} {path}: invalid JSON: {err}")))?;
        normalize::rest_json(path, &mut value);
        (Some(canonical(value)), None)
    } else {
        (None, Some(normalize::text(&body)))
    };
    Ok(RestRecord {
        method: method.to_string(),
        path: path.to_string(),
        request_headers,
        status: response.status,
        headers,
        json,
        text,
    })
}

/// Runs every CLI case against `cli_bin` and returns the records.
fn run_cli(cli_bin: &Path) -> io::Result<Vec<(String, Vec<CliRecord>)>> {
    let home = empty_home()?;
    let groups = cli_groups()
        .into_iter()
        .map(|group| {
            let records = group
                .cases
                .iter()
                .map(|args| cli_run(cli_bin, args, &home))
                .collect::<io::Result<Vec<_>>>()?;
            Ok((group.name, records))
        })
        .collect::<io::Result<Vec<_>>>();
    fs::remove_dir_all(&home)?;
    groups
}

fn cli_run(cli_bin: &Path, args: &[String], home: &Path) -> io::Result<CliRecord> {
    let output = run::cli(cli_bin, args, home)?;
    let (stdout_json, stdout) = if output.exit_code != 0 {
        (None, None)
    } else if let Ok(value) = serde_json::from_str::<Value>(&output.stdout) {
        (Some(canonical(value)), None)
    } else {
        (None, Some(normalize::text(&output.stdout)))
    };
    Ok(CliRecord {
        args: args.to_vec(),
        exit_code: output.exit_code,
        stdout_json,
        stdout,
    })
}

fn empty_home() -> io::Result<PathBuf> {
    let home = std::env::temp_dir().join(format!("pgconfig-golden-home-{}", std::process::id()));
    fs::create_dir_all(&home)?;
    Ok(home)
}

/// Records the goldens under `dir` from the given binaries. A binary that is
/// not given leaves its files untouched.
pub fn record(dir: &Path, server_bin: Option<&Path>, cli_bin: Option<&Path>) -> io::Result<()> {
    if let Some(server_bin) = server_bin {
        write_groups(&dir.join("rest"), run_rest(server_bin)?)?;
    }
    if let Some(cli_bin) = cli_bin {
        write_groups(&dir.join("cli"), run_cli(cli_bin)?)?;
    }
    Ok(())
}

fn write_groups<T: record::Record>(dir: &Path, groups: Vec<(String, Vec<T>)>) -> io::Result<()> {
    // Drop files of groups that no longer exist.
    if dir.exists() {
        fs::remove_dir_all(dir)?;
    }
    for (name, records) in groups {
        record::write_records(&dir.join(format!("{name}.jsonl")), &records)?;
    }
    Ok(())
}

/// What a check found.
#[derive(Debug, Default)]
pub struct Report {
    /// How many records were replayed.
    pub checked: usize,
    /// One message per record that differs from its golden.
    pub mismatches: Vec<String>,
}

impl Report {
    pub fn passed(&self) -> bool {
        self.mismatches.is_empty()
    }

    /// A summary that fits a test failure: the count and the first few
    /// mismatches.
    pub fn summary(&self) -> String {
        let mut out = format!(
            "{} of {} records differ",
            self.mismatches.len(),
            self.checked
        );
        for mismatch in self.mismatches.iter().take(10) {
            out.push_str("\n\n");
            out.push_str(mismatch);
        }
        if self.mismatches.len() > 10 {
            out.push_str(&format!("\n\n... and {} more", self.mismatches.len() - 10));
        }
        out
    }
}

/// Replays every stored REST record against `server_bin`.
pub fn check_server(dir: &Path, server_bin: &Path) -> io::Result<Report> {
    let stored = stored_rest(dir)?;
    let mut report = Report::default();
    report.mismatches.extend(stale(
        "rest",
        stored.iter().map(|(name, records)| {
            (
                name.as_str(),
                records
                    .iter()
                    .map(|r| format!("{} {}", r.method, r.path))
                    .collect(),
            )
        }),
        rest_groups().iter().map(|group| {
            (
                group.name.as_str(),
                group
                    .cases
                    .iter()
                    .map(|c| format!("{} {}", c.method, c.path))
                    .collect(),
            )
        }),
    ));

    let server = run::Server::spawn(server_bin, &repo_root())?;
    let mut client = server.client();
    for (name, records) in in_recording_order(&stored) {
        for expected in records {
            let actual = rest_exchange(
                &mut client,
                &expected.method,
                &expected.path,
                expected.request_headers.clone(),
                !expected.headers.is_empty(),
            )?;
            report.checked += 1;
            if &actual != expected {
                report.mismatches.push(format!(
                    "rest/{name}.jsonl: {} {}\n{}",
                    expected.method,
                    expected.path,
                    difference(expected, &actual)
                ));
            }
        }
    }
    Ok(report)
}

/// The stored REST files in the order they were recorded, which differs from
/// their order on disk. The Go API leaks state between requests (see
/// `cases::docs`), so a replay only reproduces a recording made in the same
/// order.
fn in_recording_order(
    stored: &[(String, Vec<RestRecord>)],
) -> impl Iterator<Item = (&String, &Vec<RestRecord>)> {
    let position = |name: &String| {
        rest_groups()
            .iter()
            .position(|group| &group.name == name)
            .unwrap_or(usize::MAX)
    };
    let mut ordered: Vec<(usize, &String, &Vec<RestRecord>)> = stored
        .iter()
        .map(|(name, records)| (position(name), name, records))
        .collect();
    ordered.sort_by_key(|(position, name, _)| (*position, *name));
    ordered
        .into_iter()
        .map(|(_, name, records)| (name, records))
}

/// Replays every stored CLI record against `cli_bin`.
pub fn check_cli(dir: &Path, cli_bin: &Path) -> io::Result<Report> {
    let stored = stored_cli(dir)?;
    let mut report = Report::default();
    report.mismatches.extend(stale(
        "cli",
        stored.iter().map(|(name, records)| {
            (
                name.as_str(),
                records.iter().map(|r| r.args.join(" ")).collect(),
            )
        }),
        cli_groups().iter().map(|group| {
            (
                group.name.as_str(),
                group.cases.iter().map(|c| c.join(" ")).collect(),
            )
        }),
    ));

    let home = empty_home()?;
    for (name, records) in &stored {
        for expected in records {
            let actual = cli_run(cli_bin, &expected.args, &home)?;
            report.checked += 1;
            if &actual != expected {
                report.mismatches.push(format!(
                    "cli/{name}.jsonl: {}\n{}",
                    expected.args.join(" "),
                    difference(expected, &actual)
                ));
            }
        }
    }
    fs::remove_dir_all(&home)?;
    Ok(report)
}

/// Reports golden files whose requests no longer match the generated cases.
fn stale<'a>(
    kind: &str,
    stored: impl Iterator<Item = (&'a str, Vec<String>)>,
    generated: impl Iterator<Item = (&'a str, Vec<String>)>,
) -> Vec<String> {
    let stored: BTreeMap<&str, Vec<String>> = stored.collect();
    let generated: BTreeMap<&str, Vec<String>> = generated.collect();
    let mut out = Vec::new();
    for (name, requests) in &generated {
        match stored.get(name) {
            None => out.push(format!(
                "{kind}/{name}.jsonl is missing: record the goldens again"
            )),
            Some(stored) if stored != requests => out.push(format!(
                "{kind}/{name}.jsonl does not hold the generated cases: record the goldens again"
            )),
            Some(_) => {}
        }
    }
    for name in stored.keys().filter(|name| !generated.contains_key(*name)) {
        out.push(format!(
            "{kind}/{name}.jsonl has no generated cases: record the goldens again"
        ));
    }
    out
}

/// Describes how two records differ, field by field.
fn difference<T: serde::Serialize>(expected: &T, actual: &T) -> String {
    let expected = serde_json::to_value(expected).unwrap_or_default();
    let actual = serde_json::to_value(actual).unwrap_or_default();
    let mut out = Vec::new();
    let fields: std::collections::BTreeSet<&String> = expected
        .as_object()
        .into_iter()
        .chain(actual.as_object())
        .flat_map(|map| map.keys())
        .collect();
    for field in fields {
        let (want, got) = (expected.get(field), actual.get(field));
        if want != got {
            out.push(format!(
                "  {field}:\n    expected: {}\n    actual:   {}",
                show(want),
                show(got)
            ));
        }
    }
    out.join("\n")
}

fn show(value: Option<&Value>) -> String {
    match value {
        None => "(absent)".to_string(),
        Some(Value::String(text)) => format!("{text:?}"),
        Some(value) => value.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn requests(groups: &[(&'static str, &[&str])]) -> Vec<(&'static str, Vec<String>)> {
        groups
            .iter()
            .map(|(name, cases)| (*name, cases.iter().map(|case| case.to_string()).collect()))
            .collect()
    }

    #[test]
    fn stored_files_that_hold_the_generated_cases_are_fresh() {
        let groups = requests(&[("routes", &["GET /v1/version"])]);

        assert!(stale("rest", groups.clone().into_iter(), groups.into_iter()).is_empty());
    }

    #[test]
    fn a_changed_missing_or_leftover_file_is_stale() {
        let stored = requests(&[("routes", &["GET /v1/version"]), ("leftover", &[])]);
        let generated = requests(&[("routes", &["GET /v1/other"]), ("new", &[])]);

        let found = stale("rest", stored.into_iter(), generated.into_iter());

        assert_eq!(
            found,
            [
                "rest/new.jsonl is missing: record the goldens again",
                "rest/routes.jsonl does not hold the generated cases: record the goldens again",
                "rest/leftover.jsonl has no generated cases: record the goldens again",
            ]
        );
    }

    #[test]
    fn a_difference_names_the_fields_that_changed() {
        let expected = CliRecord {
            args: vec!["tune".into()],
            exit_code: 0,
            stdout_json: None,
            stdout: Some("work_mem = 5MB\n".into()),
        };
        let actual = CliRecord {
            exit_code: 2,
            stdout: None,
            ..expected.clone()
        };

        assert_eq!(
            difference(&expected, &actual),
            "  exit_code:\n    expected: 0\n    actual:   2\n  stdout:\n    expected: \"work_mem = 5MB\\n\"\n    actual:   (absent)"
        );
    }

    #[test]
    fn generated_case_names_are_unique() {
        let mut names: Vec<String> = rest_groups().into_iter().map(|group| group.name).collect();
        names.extend(
            cli_groups()
                .into_iter()
                .map(|group| format!("cli-{}", group.name)),
        );
        let total = names.len();
        names.sort();
        names.dedup();

        assert_eq!(names.len(), total);
    }
}
