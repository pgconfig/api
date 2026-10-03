//! The quoted records in PostgreSQL 19's guc_parameters.dat. This reads
//! data only; it never evaluates Perl or C code.

use std::collections::BTreeMap;

pub(crate) fn records(source: &str) -> Result<Vec<BTreeMap<String, String>>, String> {
    let mut parser = Parser(source);
    parser.take("[")?;
    let mut records = Vec::new();
    while !parser.rest().starts_with(']') {
        parser.take("{")?;
        let mut fields = BTreeMap::new();
        while !parser.rest().starts_with('}') {
            let length = parser
                .rest()
                .find(|c: char| !c.is_ascii_alphanumeric() && c != '_')
                .unwrap_or(parser.0.len());
            if length == 0 {
                return Err("expected a field name in guc_parameters.dat".into());
            }
            let name = parser.0[..length].to_string();
            parser.0 = &parser.0[length..];
            parser.take("=>")?;
            let value = parser.quoted()?;
            if fields.insert(name.clone(), value).is_some() {
                return Err(format!("duplicate field {name} in guc_parameters.dat"));
            }
            if parser.rest().starts_with(',') {
                parser.take(",")?;
            } else if !parser.rest().starts_with('}') {
                return Err("expected a comma in guc_parameters.dat".into());
            }
        }
        parser.take("}")?;
        records.push(fields);
        if parser.rest().starts_with(',') {
            parser.take(",")?;
        } else if !parser.rest().starts_with(']') {
            return Err("expected a comma between GUC records".into());
        }
    }
    parser.take("]")?;
    if !parser.rest().is_empty() {
        return Err("unexpected text after guc_parameters.dat".into());
    }
    Ok(records)
}

struct Parser<'a>(&'a str);

impl Parser<'_> {
    fn rest(&mut self) -> &str {
        loop {
            self.0 = self.0.trim_start();
            if self.0.starts_with('#') {
                self.0 = self.0.split_once('\n').map_or("", |(_, rest)| rest);
            } else if self.0.starts_with("/*")
                && let Some((_, rest)) = self.0.split_once("*/")
            {
                self.0 = rest;
            } else {
                return self.0;
            }
        }
    }

    fn take(&mut self, token: &str) -> Result<(), String> {
        self.rest();
        self.0 = self.0.strip_prefix(token).ok_or_else(|| {
            format!(
                "expected {token:?} in guc_parameters.dat near {:?}",
                self.0.chars().take(80).collect::<String>()
            )
        })?;
        Ok(())
    }

    fn quoted(&mut self) -> Result<String, String> {
        self.take("'")?;
        let mut value = String::new();
        let mut chars = self.0.char_indices().peekable();
        while let Some((at, ch)) = chars.next() {
            match ch {
                '\'' => {
                    self.0 = &self.0[at + 1..];
                    return Ok(value);
                }
                // Perl single-quoted strings only unescape quotes and
                // backslashes. Other escapes still belong to C literals.
                '\\' if chars
                    .peek()
                    .is_some_and(|(_, next)| matches!(next, '\\' | '\'')) =>
                {
                    value.push(chars.next().unwrap().1);
                }
                _ => value.push(ch),
            }
        }
        Err("an unclosed string in guc_parameters.dat".into())
    }
}

#[cfg(test)]
mod tests {
    use super::records;

    #[test]
    fn reads_hash_and_c_comments_between_records_and_fields() {
        let parsed = records("[ # heading\n/* see max_wal_senders */\n{ name => 'pool', /* default */ boot_val => '2' }, ]").unwrap();
        assert_eq!(parsed[0]["name"], "pool");
        assert_eq!(parsed[0]["boot_val"], "2");
    }

    #[test]
    fn rejects_duplicate_fields_unclosed_strings_and_unexpected_input() {
        for invalid in [
            "[{ name => 'one', name => 'two' }]",
            "[{ name => 'open }]",
            "[{ name => 'one' }] garbage",
            "[/* open",
            "[{ name => 'one' } { name => 'two' }]",
        ] {
            assert!(records(invalid).is_err(), "accepted {invalid}");
        }
    }
}
