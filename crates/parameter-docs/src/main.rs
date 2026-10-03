//! Extracts the PostgreSQL parameter documentation into `parameters/`.
//!
//! ```text
//! parameter-docs [--postgres <checkout>] [<major>...]
//! ```
//!
//! The checkout defaults to `$PG_CHECKOUT_DIR`. The files go to `parameters/`
//! at the repository root. Without a major version, every supported one is
//! extracted.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use pgconfig_parameter_docs::git::Checkout;
use pgconfig_parameter_docs::{SUPPORTED, corpus};

const USAGE: &str = "Usage: parameter-docs [--postgres <checkout>] [<major>...]";

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("{message}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), String> {
    let mut args = std::env::args().skip(1);
    let mut postgres = std::env::var_os("PG_CHECKOUT_DIR").map(PathBuf::from);
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../parameters");
    let mut majors = Vec::new();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--postgres" => postgres = Some(args.next().map(PathBuf::from).ok_or(USAGE)?),
            major if SUPPORTED.contains(&major) => majors.push(major.to_string()),
            _ => return Err(USAGE.to_string()),
        }
    }
    let postgres = postgres.ok_or("Set PG_CHECKOUT_DIR or pass --postgres <checkout>.")?;
    if majors.is_empty() {
        majors = SUPPORTED.iter().map(|major| major.to_string()).collect();
    }

    let checkout = Checkout::open(&postgres)?;
    for major in &majors {
        let release = pgconfig_parameter_docs::extract(&checkout, major)?;
        corpus::write(&dir, major, &release)?;
        println!(
            "{major} ({}): {} parameters, {} that a standard build leaves out: {}",
            release.tag,
            release.parameters.len(),
            release.without_settings.len(),
            release.without_settings.join(", ")
        );
    }
    Ok(())
}
