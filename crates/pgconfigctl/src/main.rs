//! `pgconfigctl`: tunes a PostgreSQL server from the command line.
//!
//! The flags, defaults, and output are the v1 contract, pinned by the goldens
//! in `tests/golden/cli`.

mod host;

use std::process::ExitCode;

use clap::error::ErrorKind;
use clap::{Args, CommandFactory, Parser, Subcommand};
use pgconfig::v1;
use pgconfig::{Profile, build};

#[derive(Parser)]
#[command(
    name = "pgconfigctl",
    about = "A tool to handle and benchmark your PostgreSQL",
    // A repeated flag takes its last value, as it always has.
    args_override_self = true
)]
struct Cli {
    /// Accepted for compatibility. The Go CLI took this flag and never read
    /// the file.
    #[arg(long, global = true, hide = true)]
    config: Option<String>,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Tunes your PostgreSQL server
    #[command(
        long_about = "Uses your server info to compute the PostgreSQL tuning aiming to give you a get-start to tune your server."
    )]
    Tune(Tune),
    /// Displays version information
    Version,
}

#[derive(Args)]
struct Tune {
    /// Operating system [default: this machine's]
    #[arg(long)]
    os: Option<String>,

    /// CPU architecture [default: this machine's]
    #[arg(long)]
    arch: Option<String>,

    /// Disk type (possible values are SSD, HDD and SAN)
    #[arg(long, short = 'D', default_value = "SSD")]
    disk_type: String,

    /// PostgreSQL Version
    #[arg(long, default_value = "18", value_parser = v1::parse_pg_version)]
    version: f32,

    /// Total logical CPU cores (includes hyperthreading) [default: this machine's]
    #[arg(long, short = 'c', value_parser = v1::parse_int)]
    cpus: Option<i64>,

    /// Max expected connections
    #[arg(long, short = 'M', default_value = "100", value_parser = v1::parse_int)]
    max_connections: i64,

    /// Include pgbadger params?
    #[arg(
        long,
        short = 'B',
        num_args = 0..=1,
        require_equals = true,
        default_value = "false",
        default_missing_value = "true",
        value_parser = parse_bool
    )]
    include_pgbadger: bool,

    /// Default log format (stderr, csvlog, syslog, jsonlog) [default: csvlog, or jsonlog from PostgreSQL 15]
    #[arg(long, short = 'L')]
    log_format: Option<String>,

    /// Total memory, such as 8GB [default: this machine's]
    #[arg(long, value_parser = parse_ram)]
    ram: Option<i64>,

    /// Tuning profile (possible values are WEB, OLTP, DW, MIXED and DESKTOP)
    #[arg(long, default_value = "WEB", value_parser = v1::parse_profile)]
    profile: Profile,

    /// config file format (possible values are unix, alter-system, stackgres, and json) - file extension also work (conf, sql, json, yaml)
    #[arg(long, short = 'F', default_value = "conf", value_parser = parse_format)]
    format: String,
}

/// The amount of memory, parsed the permissive v1 way. It never fails.
fn parse_ram(text: &str) -> Result<i64, std::convert::Infallible> {
    Ok(v1::parse_bytes(text))
}

fn parse_format(text: &str) -> Result<String, String> {
    let format = text.to_lowercase();
    if v1::EXPORT_FORMATS.contains(&format.as_str()) {
        Ok(format)
    } else {
        Err(format!("must be one of [{}]", v1::EXPORT_FORMATS.join(" ")))
    }
}

/// The spellings Go's `strconv.ParseBool` accepted.
fn parse_bool(text: &str) -> Result<bool, String> {
    match text {
        "1" | "t" | "T" | "true" | "TRUE" | "True" => Ok(true),
        "0" | "f" | "F" | "false" | "FALSE" | "False" => Ok(false),
        _ => Err(format!("{text:?} is not true or false")),
    }
}

fn main() -> ExitCode {
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(error) => {
            let _ = error.print();
            // Help is a success. Any other parse error exits 1, the code the
            // Go CLI used for a bad flag.
            return match error.kind() {
                ErrorKind::DisplayHelp | ErrorKind::DisplayVersion => ExitCode::SUCCESS,
                _ => ExitCode::from(1),
            };
        }
    };

    match cli.command {
        None => {
            let _ = Cli::command().print_help();
            ExitCode::SUCCESS
        }
        Some(Command::Version) => {
            println!("pgconfigctl - {}", build::pretty());
            ExitCode::SUCCESS
        }
        Some(Command::Tune(args)) => tune(args),
    }
}

fn tune(args: Tune) -> ExitCode {
    let (total_ram, total_cpu) = match (args.ram, args.cpus) {
        (Some(ram), Some(cpus)) => (ram, cpus),
        (ram, cpus) => {
            let (host_ram, host_cpus) = host::memory_and_cpus();
            (ram.unwrap_or(host_ram), cpus.unwrap_or(host_cpus))
        }
    };
    let input = v1::Input {
        pg_version: args.version,
        total_ram,
        total_cpu,
        max_connections: args.max_connections,
        profile: args.profile,
        os: args.os.unwrap_or_else(|| host::os().to_string()),
        arch: args.arch.unwrap_or_else(|| host::arch().to_string()),
        drive_type: args.disk_type,
    };

    // PostgreSQL 15 added jsonlog, which pgbadger reads best.
    let log_format = args.log_format.unwrap_or_else(|| {
        if input.pg_version >= 15.0 {
            "jsonlog"
        } else {
            "csvlog"
        }
        .to_string()
    });
    let pgbadger = args.include_pgbadger.then_some(log_format.as_str());

    match v1::categories(&input, pgbadger) {
        Ok(categories) => {
            let export = v1::export(
                &args.format,
                &categories,
                input.pg_version,
                &build::pretty(),
                &[],
            );
            println!("{export}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("Error: {error}");
            // The Go CLI panicked here, which exits 2.
            ExitCode::from(2)
        }
    }
}
