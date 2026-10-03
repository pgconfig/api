//! The documentation that ships with the crate: what `rules.yml` says about
//! each setting, and what `pg-docs.yml` records from the PostgreSQL manual for
//! each release.

/// The pgconfig notes on one setting.
pub(crate) struct RuleDoc {
    pub abstract_text: &'static str,
    /// Title and URL of further reading, sorted by title.
    pub recomendations: &'static [(&'static str, &'static str)],
}

/// The PostgreSQL manual entry of one setting in one release.
pub(crate) struct ParamDoc {
    pub title: &'static str,
    pub short_desc: &'static str,
    pub details: &'static [&'static str],
    pub url: &'static str,
    pub conf_url: &'static str,
    pub recomendations_conf: &'static str,
    pub param_type: &'static str,
    pub default_value: &'static str,
    pub min_value: &'static str,
    pub max_value: &'static str,
}

include!(concat!(env!("OUT_DIR"), "/data.rs"));

fn find<T>(table: &'static [(&str, &[(&str, T)])], outer: &str, inner: &str) -> Option<&'static T> {
    let (_, entries) = table.iter().find(|(key, _)| *key == outer)?;
    entries
        .iter()
        .find(|(key, _)| *key == inner)
        .map(|(_, value)| value)
}

/// The notes on `parameter`, looked up under its v1 category.
pub(crate) fn rule(category: &str, parameter: &str) -> Option<&'static RuleDoc> {
    find(RULES, category, parameter)
}

/// The manual entry of `parameter` in the release keyed `version`, such as
/// `"9.6"` or `"18"`.
pub(crate) fn param(version: &str, parameter: &str) -> Option<&'static ParamDoc> {
    find(PG_DOCS, version, parameter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_legacy_release_documents_shared_buffers() {
        for (major, _) in PG_DOCS {
            let doc = param(major, "shared_buffers").expect("a manual entry");
            assert_eq!(doc.title, "shared_buffers");
            assert!(doc.url.contains("postgresql.org"), "{major}: {}", doc.url);
        }
    }

    #[test]
    fn rules_keep_markdown_and_links() {
        let doc = rule("memory_related", "work_mem").expect("the work_mem notes");

        assert!(doc.abstract_text.contains("> [!WARNING]"));
        assert!(
            doc.recomendations
                .iter()
                .all(|(_, url)| url.starts_with("http"))
        );
        assert!(doc.recomendations.is_sorted());
    }

    #[test]
    fn an_unknown_key_has_no_documentation() {
        assert!(param("19", "shared_buffers").is_none());
        assert!(rule("log_config", "logging_collector").is_none());
    }
}
