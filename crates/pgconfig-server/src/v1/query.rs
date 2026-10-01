//! The query string, read the two ways the Go API read it.
//!
//! Fiber took each argument from fasthttp, which decodes leniently and
//! returns the first value of a repeated key. The `meta.arguments` object
//! came from Go's `url.ParseQuery`, which is strict and keeps every value.
//! Clients see both, so both are reproduced.

use std::collections::BTreeMap;

/// A raw query string, without the leading `?`.
#[derive(Clone, Copy)]
pub(crate) struct Query<'a>(pub &'a str);

impl Query<'_> {
    /// The first value of `key`, or `None` when the key is absent or its
    /// value is empty. The Go API applied its default in both cases.
    pub(crate) fn get(&self, key: &str) -> Option<String> {
        self.0
            .split('&')
            .map(|pair| pair.split_once('=').unwrap_or((pair, "")))
            .find(|(name, _)| decode_lenient(name) == key)
            .map(|(_, value)| decode_lenient(value))
            .filter(|value| !value.is_empty())
    }

    /// Every argument with every value, as `meta.arguments` lists them. A
    /// pair with a semicolon or a malformed escape is dropped.
    pub(crate) fn arguments(&self) -> BTreeMap<String, Vec<String>> {
        let mut arguments: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for pair in self.0.split('&') {
            if pair.is_empty() || pair.contains(';') {
                continue;
            }
            let (key, value) = pair.split_once('=').unwrap_or((pair, ""));
            if let (Some(key), Some(value)) = (decode_strict(key), decode_strict(value)) {
                arguments.entry(key).or_default().push(value);
            }
        }
        arguments
    }
}

/// Decodes `+` and `%XX`. A `%` that does not start a valid escape stays as
/// it is.
fn decode_lenient(text: &str) -> String {
    decode(text, false).unwrap_or_default()
}

/// Decodes `+` and `%XX`. A `%` that does not start a valid escape makes the
/// whole text invalid.
fn decode_strict(text: &str) -> Option<String> {
    decode(text, true)
}

fn decode(text: &str, strict: bool) -> Option<String> {
    let bytes = text.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        match bytes[index] {
            b'+' => out.push(b' '),
            b'%' => {
                let escaped = bytes
                    .get(index + 1..index + 3)
                    .and_then(|hex| std::str::from_utf8(hex).ok())
                    .filter(|hex| hex.bytes().all(|b| b.is_ascii_hexdigit()))
                    .and_then(|hex| u8::from_str_radix(hex, 16).ok());
                match escaped {
                    Some(byte) => {
                        out.push(byte);
                        index += 2;
                    }
                    None if strict => return None,
                    None => out.push(b'%'),
                }
            }
            byte => out.push(byte),
        }
        index += 1;
    }
    Some(String::from_utf8_lossy(&out).into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_returns_the_first_decoded_value() {
        let query = Query("drive_type=SSD&drive_type=HDD&total_ram=2%20GB&name=O%4CTP&x=a+b");

        assert_eq!(query.get("drive_type").as_deref(), Some("SSD"));
        assert_eq!(query.get("total_ram").as_deref(), Some("2 GB"));
        assert_eq!(query.get("name").as_deref(), Some("OLTP"));
        assert_eq!(query.get("x").as_deref(), Some("a b"));
    }

    #[test]
    fn an_empty_or_absent_value_is_none() {
        let query = Query("show_doc&format=&&pg_version=17");

        assert_eq!(query.get("show_doc"), None);
        assert_eq!(query.get("format"), None);
        assert_eq!(query.get("missing"), None);
        assert_eq!(query.get("pg_version").as_deref(), Some("17"));
    }

    #[test]
    fn get_keeps_a_malformed_escape() {
        assert_eq!(Query("x=%zz").get("x").as_deref(), Some("%zz"));
        assert_eq!(Query("x=100%").get("x").as_deref(), Some("100%"));
    }

    #[test]
    fn arguments_keep_every_value() {
        let arguments = Query("drive_type=SSD&drive_type=HDD&y&also%20unknown=a%26b").arguments();

        assert_eq!(
            arguments,
            BTreeMap::from([
                ("also unknown".to_string(), vec!["a&b".to_string()]),
                (
                    "drive_type".to_string(),
                    vec!["SSD".to_string(), "HDD".to_string()]
                ),
                ("y".to_string(), vec![String::new()]),
            ])
        );
    }

    #[test]
    fn arguments_drop_what_go_could_not_parse() {
        let arguments = Query("x=%zz&&a=1;b=2&k=%41&=empty").arguments();

        assert_eq!(
            arguments,
            BTreeMap::from([
                (String::new(), vec!["empty".to_string()]),
                ("k".to_string(), vec!["A".to_string()]),
            ])
        );
    }
}
