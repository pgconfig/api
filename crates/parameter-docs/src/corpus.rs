//! The files of `parameters/`: one Markdown file per parameter and major
//! version, with the settings as YAML front matter.

use std::fs;
use std::path::Path;

use crate::guc::Setting;
use crate::{Release, SUPPORTED};

/// The file that records the release tag each version comes from.
const SOURCES: &str = "sources.yml";

/// One parameter of one major version.
pub struct Parameter {
    pub name: String,
    pub version: String,
    /// The type the manual states, used when the build lacks the parameter.
    pub param_type: Option<String>,
    pub url: String,
    /// The manual's description, in Markdown.
    pub text: String,
    /// The settings from the GUC tables, or `None` when a standard build
    /// does not have the parameter.
    pub setting: Option<Setting>,
}

/// The file of `parameter`. Every value is a JSON string, which YAML reads
/// as the same string: `on` stays text, and so does `010`.
pub fn render(parameter: &Parameter) -> String {
    let mut fields: Vec<(&str, String)> = vec![
        ("name", quote(&parameter.name)),
        ("version", quote(&parameter.version)),
    ];
    let setting = parameter.setting.as_ref();
    // The tables name one of five types. The manual names the SQL type of a
    // few parameters, such as pg_lsn, so it only counts without a setting.
    let param_type = setting
        .map(|setting| setting.vartype.as_str())
        .or(parameter.param_type.as_deref());
    if let Some(param_type) = param_type {
        fields.push(("type", quote(param_type)));
    }
    if let Some(setting) = setting {
        fields.push(("category", quote(&setting.category)));
        fields.push(("short_desc", quote(&setting.short_desc)));
        let optional = [
            ("extra_desc", &setting.extra_desc),
            ("context", &Some(setting.context.clone())),
            ("unit", &setting.unit),
            ("default", &setting.default),
            ("min", &setting.min),
            ("max", &setting.max),
        ];
        for (key, value) in optional {
            if let Some(value) = value {
                fields.push((key, quote(value)));
            }
        }
        if !setting.values.is_empty() {
            let values: Vec<String> = setting.values.iter().map(|value| quote(value)).collect();
            fields.push(("values", format!("[{}]", values.join(", "))));
        }
    }
    fields.push(("url", quote(&parameter.url)));

    let mut file = String::from("---\n");
    for (key, value) in fields {
        file.push_str(&format!("{key}: {value}\n"));
    }
    file.push_str("---\n\n");
    file.push_str(&parameter.text);
    file.push('\n');
    file
}

fn quote(value: &str) -> String {
    serde_json::to_string(value).expect("a string is JSON")
}

/// Replaces the files of one major version in `dir`, records its release
/// tag in `sources.yml`, and copies the PostgreSQL copyright notice.
pub fn write(dir: &Path, major: &str, release: &Release) -> Result<(), String> {
    let io = |path: &Path, err: std::io::Error| format!("{}: {err}", path.display());
    let version_dir = dir.join(major);
    if version_dir.exists() {
        for entry in fs::read_dir(&version_dir).map_err(|err| io(&version_dir, err))? {
            let path = entry.map_err(|err| io(&version_dir, err))?.path();
            if path.extension().is_some_and(|extension| extension == "md") {
                fs::remove_file(&path).map_err(|err| io(&path, err))?;
            }
        }
    }
    fs::create_dir_all(&version_dir).map_err(|err| io(&version_dir, err))?;
    for parameter in &release.parameters {
        // Names are case-insensitive, and some are written in mixed case,
        // such as DateStyle: the file takes the lowercase name.
        let file = parameter.name.to_ascii_lowercase();
        let named = !file.is_empty()
            && file
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_');
        if !named {
            return Err(format!("{:?} cannot name a file", parameter.name));
        }
        let path = version_dir.join(format!("{file}.md"));
        fs::write(&path, render(parameter)).map_err(|err| io(&path, err))?;
    }

    let sources_path = dir.join(SOURCES);
    let existing = fs::read_to_string(&sources_path).unwrap_or_default();
    let tag_of = |version: &str| {
        if version == major {
            return Some(release.tag.clone());
        }
        let key = quote(version);
        existing.lines().find_map(|line| {
            let tag = line.strip_prefix(&key)?.strip_prefix(':')?.trim();
            Some(tag.to_string())
        })
    };
    let mut sources =
        String::from("# The PostgreSQL release tag each directory was extracted from.\n");
    for version in SUPPORTED {
        if let Some(tag) = tag_of(version) {
            sources.push_str(&format!("{}: {tag}\n", quote(version)));
        }
    }
    fs::write(&sources_path, sources).map_err(|err| io(&sources_path, err))?;

    // The notice of the newest release covers the text of every release.
    if major == SUPPORTED[SUPPORTED.len() - 1] {
        let path = dir.join("COPYRIGHT");
        fs::write(&path, &release.copyright).map_err(|err| io(&path, err))?;
    }
    Ok(())
}
