//! Records or checks the golden files.
//!
//! ```text
//! golden record --server-bin <path> --cli-bin <path> [--dir <path>]
//! golden check  --server-bin <path> --cli-bin <path> [--dir <path>]
//! ```
//!
//! Either binary can be left out to record or check only the other one.

use std::path::PathBuf;
use std::process::ExitCode;

const USAGE: &str =
    "Usage: golden <record|check> [--server-bin <path>] [--cli-bin <path>] [--dir <path>]";

fn main() -> ExitCode {
    match run() {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::from(1),
        Err(message) => {
            eprintln!("{message}");
            ExitCode::from(2)
        }
    }
}

fn run() -> Result<bool, String> {
    let mut args = std::env::args().skip(1);
    let command = args.next().ok_or(USAGE)?;
    let (mut server_bin, mut cli_bin, mut dir) = (None, None, None);
    while let Some(flag) = args.next() {
        let value = args.next().map(PathBuf::from).ok_or(USAGE)?;
        match flag.as_str() {
            "--server-bin" => server_bin = Some(value),
            "--cli-bin" => cli_bin = Some(value),
            "--dir" => dir = Some(value),
            _ => return Err(USAGE.to_string()),
        }
    }
    if server_bin.is_none() && cli_bin.is_none() {
        return Err(USAGE.to_string());
    }
    let dir = dir.unwrap_or_else(pgconfig_golden::golden_dir);

    match command.as_str() {
        "record" => {
            pgconfig_golden::record(&dir, server_bin.as_deref(), cli_bin.as_deref())
                .map_err(|err| err.to_string())?;
            println!("recorded the goldens in {}", dir.display());
            Ok(true)
        }
        "check" => {
            let mut passed = true;
            if let Some(server_bin) = &server_bin {
                let report = pgconfig_golden::check_server(&dir, server_bin)
                    .map_err(|err| err.to_string())?;
                println!("{}: {}", server_bin.display(), report.summary());
                passed &= report.passed();
            }
            if let Some(cli_bin) = &cli_bin {
                let report =
                    pgconfig_golden::check_cli(&dir, cli_bin).map_err(|err| err.to_string())?;
                println!("{}: {}", cli_bin.display(), report.summary());
                passed &= report.passed();
            }
            Ok(passed)
        }
        _ => Err(USAGE.to_string()),
    }
}
