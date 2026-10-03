//! Extracts the documentation of every PostgreSQL configuration parameter from
//! a PostgreSQL git checkout into `parameters/`.
//!
//! The text comes from `doc/src/sgml/config.sgml` and the settings (context,
//! unit, default, limits) from the GUC tables in C, both read at the release
//! tag of each major version.

mod c;
pub mod corpus;
pub mod git;
pub mod guc;
pub mod manual;
mod markup;
mod platform;

/// The PostgreSQL major versions pgconfig supports, oldest first, as
/// `PgMajor::supported` lists them.
pub const SUPPORTED: [&str; 15] = [
    "9.1", "9.2", "9.3", "9.4", "9.5", "9.6", "10", "11", "12", "13", "14", "15", "16", "17", "18",
];

/// Everything one release documents about its parameters.
pub struct Release {
    pub tag: String,
    pub parameters: Vec<corpus::Parameter>,
    /// The parameters the manual documents that a standard build leaves out.
    pub without_settings: Vec<String>,
    /// The `COPYRIGHT` file of the release.
    pub copyright: String,
}

/// Reads the parameters of the major version `major` at its newest release
/// tag.
pub fn extract(checkout: &git::Checkout, major: &str) -> Result<Release, String> {
    let tag = checkout.release_tag(major)?;
    let at = |err: String| format!("{tag}: {err}");
    let mut reader = checkout.reader()?;
    let mut read = |path: &str| -> Result<String, String> {
        reader
            .read(&tag, path)?
            .ok_or_else(|| format!("{tag} has no {path}"))
    };

    let mut files = Vec::new();
    for path in checkout.files(&tag, "doc/src/sgml")? {
        if path.ends_with(".sgml") {
            let source = read(&path)?;
            files.push((path, source));
        }
    }
    let book = manual::Book::index(
        files
            .iter()
            .map(|(path, source)| (path.as_str(), source.as_str())),
    )
    .map_err(at)?;
    let config = &files
        .iter()
        .find(|(path, _)| path == "doc/src/sgml/config.sgml")
        .ok_or_else(|| at("there is no config.sgml".into()))?
        .1;
    let entries = manual::entries(config, &book, major).map_err(at)?;

    let tables_path = if checkout
        .files(&tag, "src/backend/utils/misc/guc_tables.c")?
        .is_empty()
    {
        "src/backend/utils/misc/guc.c"
    } else {
        "src/backend/utils/misc/guc_tables.c"
    };
    let tables = read(tables_path)?;
    let header = read("src/include/utils/guc_tables.h")?;
    let mut headers = Vec::new();
    for path in checkout.files(&tag, "src/include")? {
        if path.ends_with(".h") && !other_platform(&path) {
            headers.push(read(&path)?);
        }
    }
    let mut option_paths: Vec<String> = checkout
        .grep(
            &tag,
            r"config_enum_entry[[:space:]]+[A-Za-z_0-9]+\[\][[:space:]]*=",
            &["src"],
        )?
        .into_iter()
        .map(|(path, _)| path)
        .filter(|path| path != tables_path)
        .collect();
    option_paths.dedup();
    let mut options = Vec::new();
    for path in &option_paths {
        options.push(read(path)?);
    }
    let options: Vec<&str> = options.iter().map(String::as_str).collect();
    let headers: Vec<&str> = headers.iter().map(String::as_str).collect();
    let settings = guc::settings(&guc::Sources {
        tables: &tables,
        header: &header,
        options: &options,
        headers: &headers,
        release: &release_number(&tag)?,
    })
    .map_err(at)?;
    // Names are case-insensitive: the manual and the tables may differ.
    let mut settings: std::collections::HashMap<String, guc::Setting> = settings
        .into_iter()
        .map(|(name, setting)| (name.to_ascii_lowercase(), setting))
        .collect();

    let copyright = read("COPYRIGHT")?;
    let mut parameters = Vec::new();
    let mut without_settings = Vec::new();
    for entry in entries {
        let setting = settings.remove(&entry.name.to_ascii_lowercase());
        if setting.is_none() {
            without_settings.push(entry.name.clone());
        }
        parameters.push(corpus::Parameter {
            name: entry.name,
            version: major.to_string(),
            param_type: entry.param_type,
            url: entry.url,
            text: entry.text,
            setting,
        });
    }
    Ok(Release {
        tag,
        parameters,
        without_settings,
        copyright,
    })
}

/// Whether a header belongs to another operating system or CPU, which a
/// Linux build never includes.
fn other_platform(path: &str) -> bool {
    let Some(port) = path.strip_prefix("src/include/port/") else {
        return false;
    };
    const OTHERS: &[&str] = &[
        "aix", "bsdi", "cygwin", "darwin", "dgux", "freebsd", "hpux", "hurd", "irix5", "netbsd",
        "nextstep", "openbsd", "osf", "qnx4", "sco", "solaris", "sunos4", "svr4", "ultrix4",
        "univel", "unixware",
    ];
    let stem = port.trim_end_matches(".h");
    port.starts_with("win32") || port.starts_with("atomics/") || OTHERS.contains(&stem)
}

/// The release a tag names: `18.6` for `REL_18_6`, `9.6.24` for `REL9_6_24`.
fn release_number(tag: &str) -> Result<String, String> {
    let digits = tag
        .strip_prefix("REL_")
        .or_else(|| tag.strip_prefix("REL"))
        .ok_or_else(|| format!("{tag} is not a release tag"))?;
    Ok(digits.replace('_', "."))
}
