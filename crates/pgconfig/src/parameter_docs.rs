//! The PostgreSQL manual's entry for every configuration parameter of every
//! supported major version, from `parameters/` at the repository root.
//!
//! REST v1 keeps its own, older copy of a few entries in `pg-docs.yml`.

use crate::PgMajor;

/// One parameter of one PostgreSQL Major Version, as the PostgreSQL manual
/// documents it and its GUC tables define it. The settings use the words of
/// `pg_settings`.
#[derive(Debug)]
pub struct ParameterDoc {
    /// The name as the manual writes it, such as `work_mem` or `DateStyle`.
    pub name: &'static str,
    /// The type in the manual's words: `boolean`, `integer`,
    /// `floating point`, `string`, or `enum`.
    pub param_type: Option<&'static str>,
    pub category: Option<&'static str>,
    pub short_desc: Option<&'static str>,
    pub extra_desc: Option<&'static str>,
    /// When a change takes effect: `postmaster` needs a restart.
    pub context: Option<&'static str>,
    pub unit: Option<&'static str>,
    pub default: Option<&'static str>,
    pub min: Option<&'static str>,
    pub max: Option<&'static str>,
    /// The values an enum accepts.
    pub values: &'static [&'static str],
    /// The entry in the PostgreSQL manual.
    pub url: &'static str,
    markdown: &'static str,
    text_start: usize,
}

impl ParameterDoc {
    /// The file in `parameters/`: YAML front matter, then the manual's text.
    pub fn markdown(&self) -> &'static str {
        self.markdown
    }

    /// The manual's text, in Markdown.
    pub fn text(&self) -> &'static str {
        self.markdown[self.text_start..].trim_end()
    }
}

include!(concat!(env!("OUT_DIR"), "/parameter_docs.rs"));

/// Every parameter the manual of `major` documents, sorted by name.
pub fn parameter_docs(major: PgMajor) -> &'static [ParameterDoc] {
    let version = major.to_string();
    PARAMETER_DOCS
        .iter()
        .find(|(documented, _)| *documented == version)
        .map_or(&[], |(_, docs)| docs)
}

/// The parameter `name` of `major`. Names are case-insensitive, as they are
/// in PostgreSQL.
pub fn parameter_doc(major: PgMajor, name: &str) -> Option<&'static ParameterDoc> {
    parameter_docs(major)
        .iter()
        .find(|doc| doc.name.eq_ignore_ascii_case(name))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PgVersion;

    #[test]
    fn every_supported_release_has_a_manual() {
        for major in PgMajor::supported() {
            let doc = parameter_doc(major, "shared_buffers").expect("a versioned manual entry");
            assert!(doc.default.is_some());
            assert!(doc.url.contains(&format!("/docs/{major}/")));
            assert!(!doc.text().is_empty());
        }
    }

    #[test]
    fn postgresql_19_documents_the_dynamic_pool_and_separate_analyze_logging() {
        let pg19 = PgVersion::parse("19").unwrap().major();
        for (name, default) in [
            ("io_min_workers", "2"),
            ("io_max_workers", "8"),
            ("log_autoanalyze_min_duration", "600000"),
        ] {
            let doc = parameter_doc(pg19, name).expect("a new PostgreSQL 19 parameter");
            assert_eq!(doc.default, Some(default));
            assert_eq!(doc.context, Some("sighup"));
            assert!(!doc.text().is_empty());
            assert!(parameter_doc(PgVersion::parse("18").unwrap().major(), name).is_none());
        }
        assert!(parameter_doc(pg19, "io_workers").is_none());
        assert!(parameter_doc(PgVersion::parse("18").unwrap().major(), "io_workers").is_some());
    }
}
