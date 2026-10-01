//! The on-disk shape of a golden record. Each golden file is JSON Lines: one
//! record per line, in the order the cases are generated.

use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::Path;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// One HTTP exchange with the server.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RestRecord {
    pub method: String,
    /// The request target exactly as sent, query string included.
    pub path: String,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub request_headers: BTreeMap<String, String>,
    pub status: u16,
    /// The pinned response headers. Empty when the case does not pin any.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub headers: BTreeMap<String, String>,
    /// A JSON body, stored parsed. Go escapes `<`, `>`, and `&` and serde_json
    /// does not, so JSON is equal when its parsed value is.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub json: Option<Value>,
    /// Any other body, compared byte for byte.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

/// One run of the CLI.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CliRecord {
    pub args: Vec<String>,
    pub exit_code: i32,
    /// Stdout of a successful run that printed JSON, stored parsed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stdout_json: Option<Value>,
    /// Stdout of a successful run that printed text. A failed run pins only
    /// its exit code: the wording of usage errors belongs to the argument
    /// parser, not to the contract.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stdout: Option<String>,
}

/// Rebuilds a JSON value with every object's keys sorted, so a record reads
/// the same whether or not serde_json keeps insertion order.
pub fn canonical(value: Value) -> Value {
    match value {
        Value::Array(items) => Value::Array(items.into_iter().map(canonical).collect()),
        Value::Object(map) => {
            let mut entries: Vec<(String, Value)> = map.into_iter().collect();
            entries.sort_by(|a, b| a.0.cmp(&b.0));
            let mut sorted = Map::new();
            for (key, value) in entries {
                sorted.insert(key, canonical(value));
            }
            Value::Object(sorted)
        }
        other => other,
    }
}

/// Reads every record of a golden file.
pub fn read_records<T: DeserializeOwned>(path: &Path) -> io::Result<Vec<T>> {
    let contents = fs::read_to_string(path)?;
    contents
        .lines()
        .enumerate()
        .map(|(index, line)| {
            serde_json::from_str(line)
                .map_err(|err| io::Error::other(format!("{}:{}: {err}", path.display(), index + 1)))
        })
        .collect()
}

/// A record that knows the order its fields are written in. The request comes
/// first so a line can be recognized without scrolling past its body.
pub trait Record: Serialize {
    const FIELDS: &'static [&'static str];
}

impl Record for RestRecord {
    const FIELDS: &'static [&'static str] = &[
        "method",
        "path",
        "request_headers",
        "status",
        "headers",
        "json",
        "text",
    ];
}

impl Record for CliRecord {
    const FIELDS: &'static [&'static str] = &["args", "exit_code", "stdout_json", "stdout"];
}

/// Writes records as JSON Lines.
pub fn write_records<T: Record>(path: &Path, records: &[T]) -> io::Result<()> {
    let mut out = String::new();
    for record in records {
        let value = serde_json::to_value(record).map_err(io::Error::other)?;
        out.push_str(&serialize_line(&value, T::FIELDS));
        out.push('\n');
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, out)
}

/// Serializes a record with its own fields in the given order and every
/// nested object in key order.
fn serialize_line(record: &Value, fields: &[&str]) -> String {
    let mut out = String::from("{");
    for field in fields {
        let Some(value) = record.get(field) else {
            continue;
        };
        if out.len() > 1 {
            out.push(',');
        }
        out.push_str(&Value::from(*field).to_string());
        out.push(':');
        out.push_str(&canonical(value.clone()).to_string());
    }
    out.push('}');
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn canonical_sorts_nested_object_keys() {
        let value: Value =
            serde_json::from_str(r#"{"b":{"z":1,"a":[{"y":1,"x":2}]},"a":0}"#).unwrap();

        assert_eq!(
            canonical(value).to_string(),
            r#"{"a":0,"b":{"a":[{"x":2,"y":1}],"z":1}}"#
        );
    }

    #[test]
    fn records_survive_a_write_and_read() {
        let dir = std::env::temp_dir().join(format!("pgconfig-golden-{}", std::process::id()));
        let path = dir.join("roundtrip.jsonl");
        let records = vec![
            RestRecord {
                method: "GET".into(),
                path: "/v1/version".into(),
                request_headers: BTreeMap::new(),
                status: 200,
                headers: BTreeMap::from([("content-type".into(), "application/json".into())]),
                json: Some(json!({"data": {"b": 1, "a": "<&>"}})),
                text: None,
            },
            RestRecord {
                method: "GET".into(),
                path: "/v1/tuning/get-config?format=conf".into(),
                request_headers: BTreeMap::new(),
                status: 200,
                headers: BTreeMap::new(),
                json: None,
                text: Some("# line\n\nshared_buffers = 1GB\n".into()),
            },
        ];

        write_records(&path, &records).unwrap();
        let read: Vec<RestRecord> = read_records(&path).unwrap();
        let written = fs::read_to_string(&path).unwrap();
        fs::remove_dir_all(&dir).unwrap();

        assert_eq!(read, records);
        assert!(written.starts_with(r#"{"method":"GET","path":"/v1/version","status":200,"#));
    }
}
