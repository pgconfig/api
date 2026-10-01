//! Replays the goldens recorded from the Go binaries through the v1
//! projection, without a server or a CLI in between.
//!
//! The server and the CLI own their argument parsing, so this test stands in
//! for them with the defaults of each interface. Their own golden tests cover
//! routing, query-string decoding, and flag syntax.

use std::collections::BTreeMap;

use pgconfig::v1;
use pgconfig_golden::{CliRecord, PROFILES, RestRecord, golden_dir, stored_cli, stored_rest};
use serde_json::{Value, json};

/// What the golden recording sent as `Host`.
const BASE_URL: &str = "http://golden.pgconfig.test";
const BUILD: &str = "<version>";

/// Groups that pin the HTTP layer, not the projection.
const SERVER_ONLY: [&str; 2] = ["routes", "query-strings"];

#[test]
fn rest_tuning_goldens_pass_through_the_v1_projection() {
    let mut checked = 0;
    let mut failures = Vec::new();
    for (group, records) in stored_rest(&golden_dir()).unwrap() {
        if SERVER_ONLY.contains(&group.as_str()) {
            continue;
        }
        for record in records {
            checked += 1;
            if let Err(problem) = check_rest(&record) {
                failures.push(format!("rest/{group}: {}\n  {problem}", record.path));
            }
        }
    }

    assert!(checked > 5800, "only {checked} records were checked");
    assert!(failures.is_empty(), "{}", report(checked, &failures));
}

#[test]
fn cli_goldens_pass_through_the_v1_projection() {
    let mut checked = 0;
    let mut failures = Vec::new();
    for (group, records) in stored_cli(&golden_dir()).unwrap() {
        for record in records {
            let Some(flags) = long_flags(&record.args) else {
                continue;
            };
            checked += 1;
            if let Err(problem) = check_cli(&record, &flags) {
                failures.push(format!(
                    "cli/{group}: {}\n  {problem}",
                    record.args.join(" ")
                ));
            }
        }
    }

    assert!(checked > 190, "only {checked} records were checked");
    assert!(failures.is_empty(), "{}", report(checked, &failures));
}

fn report(checked: usize, failures: &[String]) -> String {
    let shown: Vec<&str> = failures.iter().take(15).map(String::as_str).collect();
    format!(
        "{} of {checked} records differ\n\n{}",
        failures.len(),
        shown.join("\n\n")
    )
}

/// The first value of each query parameter, decoded. An empty value counts as
/// absent, as it did in the Go API.
fn query(path: &str) -> BTreeMap<String, String> {
    let mut values = BTreeMap::new();
    let Some((_, query)) = path.split_once('?') else {
        return values;
    };
    for pair in query.split('&') {
        let (key, value) = pair.split_once('=').unwrap_or((pair, ""));
        let value = decode(value);
        if !value.is_empty() {
            values.entry(decode(key)).or_insert(value);
        }
    }
    values
}

fn decode(text: &str) -> String {
    let bytes = text.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        let hex = bytes
            .get(index + 1..index + 3)
            .and_then(|hex| std::str::from_utf8(hex).ok());
        match (
            bytes[index],
            hex.and_then(|hex| u8::from_str_radix(hex, 16).ok()),
        ) {
            (b'%', Some(byte)) => {
                out.push(byte);
                index += 3;
                continue;
            }
            (b'+', _) => out.push(b' '),
            (byte, _) => out.push(byte),
        }
        index += 1;
    }
    String::from_utf8(out).unwrap()
}

struct Call {
    input: v1::Input,
    format: String,
    show_doc: bool,
    pgbadger: Option<String>,
}

/// Builds the call the Go API made for a query string: same defaults, same
/// order of parsing.
fn rest_call(values: &BTreeMap<String, String>) -> Result<Call, String> {
    let get = |key: &str, default: &str| values.get(key).cloned().unwrap_or(default.to_string());
    let pg_version = v1::parse_pg_version(&get("pg_version", "18")).map_err(|e| e.to_string())?;
    let max_connections =
        v1::parse_int(&get("max_connections", "100")).map_err(|e| e.to_string())?;
    let total_cpu = v1::parse_int(&get("cpus", "2")).map_err(|e| e.to_string())?;
    let total_ram = v1::parse_bytes(&get("total_ram", "2GB"));
    let profile = v1::parse_profile(&get("environment_name", "WEB")).map_err(|e| e.to_string())?;
    let default_log_format = if pg_version >= 15.0 {
        "jsonlog"
    } else {
        "stderr"
    };
    Ok(Call {
        input: v1::Input {
            pg_version,
            total_ram,
            total_cpu,
            max_connections,
            profile,
            os: get("os_type", "linux"),
            arch: get("arch", "amd64"),
            drive_type: get("drive_type", "HDD"),
        },
        format: get("format", "json"),
        show_doc: get("show_doc", "false") == "true",
        pgbadger: (get("include_pgbadger", "false") == "true")
            .then(|| get("log_format", default_log_format)),
    })
}

fn check_rest(record: &RestRecord) -> Result<(), String> {
    let route = record.path.split('?').next().unwrap_or_default();
    let all_environments = match route {
        "/v1/tuning/get-config" => false,
        "/v1/tuning/get-config-all-environments" => true,
        other => return Err(format!("unexpected route {other}")),
    };
    let outcome = rest_call(&query(&record.path)).and_then(|call| {
        if all_environments {
            respond_all_environments(call)
        } else {
            respond(call, &record.path)
        }
    });

    match (outcome, record.status) {
        (Ok(Response::Json(data)), 200) => {
            let expected = record.json.as_ref().and_then(|body| body.get("data"));
            same(expected, Some(&data), "data")
        }
        (Ok(Response::Text(text)), 200) => same(record.text.as_ref(), Some(&text), "body"),
        (Err(message), 500) => {
            let expected = record
                .json
                .as_ref()
                .and_then(|body| body.pointer("/errors/message"))
                .and_then(Value::as_str)
                .unwrap_or_default();
            if expected.ends_with(&message) {
                Ok(())
            } else {
                Err(format!("expected the error {expected:?}, got {message:?}"))
            }
        }
        (Ok(_), status) => Err(format!(
            "expected status {status}, the projection succeeded"
        )),
        (Err(message), status) => Err(format!("expected status {status}, got the error {message}")),
    }
}

enum Response {
    Json(Value),
    Text(String),
}

fn respond(call: Call, path: &str) -> Result<Response, String> {
    let mut categories =
        v1::categories(&call.input, call.pgbadger.as_deref()).map_err(|e| e.to_string())?;
    if call.show_doc {
        v1::add_documentation(&mut categories, call.input.pg_version);
    }
    if call.format == "json" {
        return Ok(Response::Json(serde_json::to_value(&categories).unwrap()));
    }
    let self_link = format!("{BASE_URL}{path}\n");
    Ok(Response::Text(v1::export(
        &call.format,
        &categories,
        call.input.pg_version,
        BUILD,
        &[self_link],
    )))
}

fn respond_all_environments(mut call: Call) -> Result<Response, String> {
    let mut data = Vec::new();
    for profile in PROFILES {
        call.input.profile = v1::parse_profile(profile).map_err(|e| e.to_string())?;
        let mut categories = v1::categories(&call.input, None).map_err(|e| e.to_string())?;
        if call.show_doc {
            v1::add_documentation(&mut categories, call.input.pg_version);
        }
        data.push(json!({"environment": profile, "configuration": categories}));
    }
    Ok(Response::Json(Value::Array(data)))
}

/// The flags of a CLI record, when every one is written as `--name value`.
/// Other spellings belong to the flag parser and are left to the CLI's test.
fn long_flags(args: &[String]) -> Option<BTreeMap<String, String>> {
    let (command, rest) = args.split_first()?;
    if command != "tune" {
        return None;
    }
    let mut flags = BTreeMap::new();
    let mut rest = rest.iter();
    while let Some(arg) = rest.next() {
        let name = arg.strip_prefix("--").filter(|name| !name.contains('='))?;
        let known = [
            "ram",
            "cpus",
            "os",
            "arch",
            "version",
            "profile",
            "disk-type",
            "max-connections",
            "log-format",
            "format",
        ];
        if name == "include-pgbadger" {
            flags.insert(name.to_string(), "true".to_string());
        } else if known.contains(&name) {
            flags.insert(name.to_string(), rest.next()?.clone());
        } else {
            return None;
        }
    }
    Some(flags)
}

fn check_cli(record: &CliRecord, flags: &BTreeMap<String, String>) -> Result<(), String> {
    let get = |key: &str, default: &str| flags.get(key).cloned().unwrap_or(default.to_string());
    let parsed = (|| -> Result<(v1::Input, String), String> {
        let input = v1::Input {
            pg_version: v1::parse_pg_version(&get("version", "18")).map_err(|e| e.to_string())?,
            total_ram: v1::parse_bytes(&get("ram", "")),
            total_cpu: v1::parse_int(&get("cpus", "")).map_err(|e| e.to_string())?,
            max_connections: v1::parse_int(&get("max-connections", "100"))
                .map_err(|e| e.to_string())?,
            profile: v1::parse_profile(&get("profile", "WEB")).map_err(|e| e.to_string())?,
            os: get("os", ""),
            arch: get("arch", ""),
            drive_type: get("disk-type", "SSD"),
        };
        let format = get("format", "conf").to_lowercase();
        if !v1::EXPORT_FORMATS.contains(&format.as_str()) {
            return Err(format!("unknown format {format}"));
        }
        Ok((input, format))
    })();

    let (input, format) = match (parsed, record.exit_code) {
        (Ok(parsed), _) => parsed,
        // The flag parser rejected a value.
        (Err(_), 1) => return Ok(()),
        (Err(message), code) => return Err(format!("expected exit code {code}, got {message}")),
    };

    let log_format = match flags.get("log-format") {
        Some(log_format) => log_format.clone(),
        None if input.pg_version >= 15.0 => "jsonlog".to_string(),
        None => "csvlog".to_string(),
    };
    let pgbadger = flags.contains_key("include-pgbadger").then_some(log_format);

    let categories = match (
        v1::categories(&input, pgbadger.as_deref()),
        record.exit_code,
    ) {
        (Ok(categories), 0) => categories,
        // The rules rejected the input, and the Go CLI panicked.
        (Err(_), 2) => return Ok(()),
        (Ok(_), code) => {
            return Err(format!(
                "expected exit code {code}, the projection succeeded"
            ));
        }
        (Err(error), code) => return Err(format!("expected exit code {code}, got {error}")),
    };

    // The CLI prints the export followed by a newline.
    let stdout = v1::export(&format, &categories, input.pg_version, BUILD, &[]) + "\n";
    if let Some(expected) = &record.stdout_json {
        let actual: Value = serde_json::from_str(&stdout).map_err(|e| e.to_string())?;
        return same(Some(expected), Some(&actual), "stdout");
    }
    same(record.stdout.as_ref(), Some(&stdout), "stdout")
}

fn same<T: PartialEq + std::fmt::Debug>(
    expected: Option<&T>,
    actual: Option<&T>,
    what: &str,
) -> Result<(), String> {
    if expected == actual {
        Ok(())
    } else {
        Err(format!(
            "{what} differs\n  expected: {expected:?}\n  actual:   {actual:?}"
        ))
    }
}
