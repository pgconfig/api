//! The version of this build, shared by the server and the CLI.

/// The release version, such as `3.6.1`.
pub const TAG: &str = env!("CARGO_PKG_VERSION");

/// The commit the release was built from. The release build sets
/// `PGCONFIG_COMMIT`; a local build says `development`.
pub const COMMIT: &str = match option_env!("PGCONFIG_COMMIT") {
    Some(commit) => commit,
    None => "development",
};

/// The version as the outputs print it: `3.6.1 (abc123)`.
pub fn pretty() -> String {
    format!("{TAG} ({COMMIT})")
}
