//! The inputs the goldens cover. Recording runs every case here against the
//! binaries. Changing this file means recording again.

pub const VERSIONS: [&str; 15] = [
    "9.1", "9.2", "9.3", "9.4", "9.5", "9.6", "10", "11", "12", "13", "14", "15", "16", "17", "18",
];
pub const PROFILES: [&str; 5] = ["WEB", "OLTP", "DW", "MIXED", "DESKTOP"];
const DISKS: [&str; 3] = ["HDD", "SSD", "SAN"];
/// Odd sizes exercise the `float32` rounding of the memory formulas. 64GB
/// reaches the caps on `shared_buffers` and the OLTP `wal_buffers` rule.
const MATRIX_RAM: [&str; 6] = ["512MB", "1536MB", "3GB", "7GB", "13GB", "64GB"];
/// 1 CPU hits the lower bounds and 128 the `io_workers` ceiling of 32.
const MATRIX_CPU: [&str; 4] = ["1", "2", "16", "128"];

pub const ORIGIN: &str = "https://golden.pgconfig.test";

pub struct RestCase {
    pub method: &'static str,
    pub path: String,
    pub request_headers: Vec<(&'static str, &'static str)>,
}

pub struct RestGroup {
    /// The golden file is `rest/<name>.jsonl`.
    pub name: String,
    /// Whether the records pin the response headers listed in `PINNED_HEADERS`.
    pub pin_headers: bool,
    pub cases: Vec<RestCase>,
}

pub struct CliGroup {
    /// The golden file is `cli/<name>.jsonl`.
    pub name: String,
    pub cases: Vec<Vec<String>>,
}

fn get(path: impl Into<String>) -> RestCase {
    RestCase {
        method: "GET",
        path: path.into(),
        request_headers: Vec::new(),
    }
}

fn method(method: &'static str, path: &str) -> RestCase {
    RestCase {
        method,
        path: path.into(),
        request_headers: Vec::new(),
    }
}

fn forwarded(path: &str, headers: &[(&'static str, &'static str)]) -> RestCase {
    RestCase {
        method: "GET",
        path: path.into(),
        request_headers: headers.to_vec(),
    }
}

fn config(query: impl AsRef<str>) -> RestCase {
    get(format!("/v1/tuning/get-config?{}", query.as_ref()))
}

fn all_environments(query: impl AsRef<str>) -> RestCase {
    get(format!(
        "/v1/tuning/get-config-all-environments?{}",
        query.as_ref()
    ))
}

pub fn rest_groups() -> Vec<RestGroup> {
    let mut groups = matrix();
    groups.extend([
        routes(),
        formats(),
        pgbadger(),
        os_and_arch(),
        connections(),
        storage(),
        aio(),
        versions(),
        ram_and_cpu(),
        query_strings(),
        all_environments_group(),
        // Last on purpose. See `docs`.
        docs(),
    ]);
    groups
}

/// The full product of version, profile, disk, RAM, and CPU, one file per
/// PostgreSQL version. The conf format carries every computed value in the
/// fewest bytes.
fn matrix() -> Vec<RestGroup> {
    VERSIONS
        .iter()
        .map(|version| {
            let mut cases = Vec::new();
            for profile in PROFILES {
                for disk in DISKS {
                    for ram in MATRIX_RAM {
                        for cpu in MATRIX_CPU {
                            cases.push(config(format!(
                                "pg_version={version}&environment_name={profile}&drive_type={disk}&total_ram={ram}&cpus={cpu}&format=conf"
                            )));
                        }
                    }
                }
            }
            RestGroup { name: format!("matrix-pg-{version}"), pin_headers: false, cases }
        })
        .collect()
}

fn routes() -> RestGroup {
    let with_origin = |mut case: RestCase| {
        case.request_headers.push(("Origin", ORIGIN));
        case
    };
    let mut preflight = method("OPTIONS", "/v1/tuning/get-config");
    preflight.request_headers = vec![("Origin", ORIGIN), ("Access-Control-Request-Method", "GET")];

    let cases = vec![
        get("/v1/version"),
        get("/v1/version?unused=1"),
        method("HEAD", "/v1/version"),
        get("/v1/tuning/list-environments"),
        get("/v1/tuning/get-config"),
        get("/v1/tuning/get-config-all-environments"),
        // Fiber routes ignore case and a trailing slash.
        get("/v1/version/"),
        get("/V1/Version"),
        get("/v1/tuning/list-environments/"),
        get("/V1/TUNING/GET-CONFIG?format=conf"),
        get("/v1/nope"),
        get("/v1/tuning/nope"),
        get("/v1/tuning"),
        method("POST", "/v1/nope"),
        method("POST", "/v1/tuning/get-config"),
        method("POST", "/v1/version"),
        method("DELETE", "/v1/tuning/list-environments"),
        // CORS headers only appear when the request has an Origin.
        with_origin(get("/v1/version")),
        with_origin(get("/v1/tuning/list-environments")),
        with_origin(config("format=conf")),
        with_origin(config("pg_version=abc")),
        preflight,
        // Behind a proxy, links.self and the conf header follow the forwarded
        // scheme and host. Production sits behind one.
        forwarded("/v1/version", &[("X-Forwarded-Proto", "https")]),
        forwarded("/v1/version", &[("X-Forwarded-Host", "api.pgconfig.org")]),
        forwarded(
            "/v1/tuning/get-config?format=conf",
            &[
                ("X-Forwarded-Host", "api.pgconfig.org"),
                ("X-Forwarded-Proto", "https"),
            ],
        ),
        forwarded("/v1/nope", &[("X-Forwarded-Proto", "https")]),
        forwarded("/v1/version", &[("X-Forwarded-Proto", "https, http")]),
        forwarded(
            "/v1/version",
            &[("X-Forwarded-Host", "a.example, b.example")],
        ),
        forwarded("/v1/version", &[("X-Forwarded-Protocol", "https")]),
        forwarded("/v1/version", &[("X-Forwarded-Ssl", "on")]),
        forwarded("/v1/version", &[("X-Forwarded-Ssl", "off")]),
        forwarded("/v1/version", &[("X-Url-Scheme", "https")]),
        forwarded(
            "/v1/version",
            &[("Forwarded", "proto=https;host=x.example")],
        ),
    ];
    RestGroup {
        name: "routes".into(),
        pin_headers: true,
        cases,
    }
}

fn formats() -> RestGroup {
    let mut cases = Vec::new();
    for version in VERSIONS {
        for format in ["json", "conf", "alter_system", "stackgres"] {
            cases.push(config(format!("pg_version={version}&format={format}")));
        }
    }
    // The remaining names the format switch accepts, and two it does not: an
    // unknown name and an uppercase one both fall through to the conf output.
    for version in ["9.6", "18"] {
        for format in [
            "sql",
            "unix",
            "sg",
            "sgpostgresconfig",
            "yaml",
            "foo",
            "JSON",
            "CONF",
        ] {
            cases.push(config(format!("pg_version={version}&format={format}")));
        }
    }
    RestGroup {
        name: "formats".into(),
        pin_headers: true,
        cases,
    }
}

fn pgbadger() -> RestGroup {
    let mut cases = Vec::new();
    // The default log format changes from stderr to jsonlog at PostgreSQL 15.
    for version in ["14", "15"] {
        for log_format in ["", "stderr", "syslog", "csvlog", "jsonlog", "foo"] {
            for format in ["json", "conf", "alter_system", "stackgres"] {
                let log = if log_format.is_empty() {
                    String::new()
                } else {
                    format!("&log_format={log_format}")
                };
                cases.push(config(format!(
                    "pg_version={version}&include_pgbadger=true&format={format}{log}"
                )));
            }
        }
    }
    cases.extend([
        config("include_pgbadger=false&log_format=syslog&format=conf"),
        config("include_pgbadger=TRUE&format=conf"),
        config("include_pgbadger=1&format=conf"),
        config("include_pgbadger=true&pg_version=9.1&format=conf"),
        config("include_pgbadger=true&pg_version=18"),
    ]);
    RestGroup {
        name: "pgbadger".into(),
        pin_headers: false,
        cases,
    }
}

/// The Go API keeps the pgbadger category in a package variable, and a request
/// with both `show_doc` and `include_pgbadger` writes the documentation into
/// it. Every later pgbadger response of that process then carries an empty
/// `documentation` object. That leak is a defect, not a contract, so the one
/// request that triggers it runs last and the goldens never record its
/// aftermath.
fn docs() -> RestGroup {
    let mut cases: Vec<RestCase> = VERSIONS
        .iter()
        .map(|version| config(format!("pg_version={version}&show_doc=true")))
        .collect();
    cases.extend([
        // The documentation lookup rounds the version: 13.5 reads "14" and
        // 12.5 reads "12". 19 and 8.4 have no documentation at all.
        config("pg_version=13.5&show_doc=true"),
        config("pg_version=12.5&show_doc=true"),
        config("pg_version=17.10&show_doc=true"),
        config("pg_version=19&show_doc=true"),
        config("pg_version=8.4&show_doc=true"),
        config("pg_version=16&show_doc=true&format=conf"),
        config("pg_version=16&show_doc=TRUE"),
        config("pg_version=16&show_doc=false"),
        all_environments("pg_version=13&show_doc=true"),
        config("pg_version=16&show_doc=true&include_pgbadger=true"),
    ]);
    RestGroup {
        name: "docs".into(),
        pin_headers: false,
        cases,
    }
}

fn os_and_arch() -> RestGroup {
    let mut cases = Vec::new();
    // 64GB over 5 connections puts work_mem and maintenance_work_mem above
    // the 2GB Windows limit of PostgreSQL 17 and older.
    for version in ["9.6", "17", "18"] {
        for os in [
            "linux", "windows", "Windows", "WINDOWS", "unix", "darwin", "Darwin", "LINUX",
        ] {
            cases.push(config(format!(
                "pg_version={version}&os_type={os}&total_ram=64GB&max_connections=5&drive_type=SSD"
            )));
        }
    }
    // 128GB over 2 connections puts the three memory settings above the 4GB
    // limit of a 32-bit build.
    for arch in ["386", "i686", "amd64", "x86-64", "arm", "arm64"] {
        cases.push(config(format!(
            "arch={arch}&total_ram=128GB&max_connections=2"
        )));
    }
    cases.extend([
        config("arch=386&os_type=windows&pg_version=9.6&total_ram=128GB&max_connections=2"),
        config("arch=i686&os_type=windows&pg_version=17&total_ram=128GB&max_connections=2"),
        config("arch=386&pg_version=9.5&total_ram=128GB&max_connections=2"),
        config("arch=386&total_ram=16385MB"),
        config("os_type=&arch="),
        config("os_type=freebsd"),
        config("os_type=freebsd&format=conf"),
        config("arch=AMD64"),
        config("arch=ppc64"),
        config("arch=ppc64&os_type=freebsd"),
    ]);
    RestGroup {
        name: "os-arch".into(),
        pin_headers: false,
        cases,
    }
}

fn connections() -> RestGroup {
    let mut cases = Vec::new();
    for profile in PROFILES {
        for ram in ["2GB", "64GB"] {
            for connections in ["1", "20", "100", "500", "10000"] {
                cases.push(config(format!(
                    "environment_name={profile}&total_ram={ram}&max_connections={connections}"
                )));
            }
        }
    }
    cases.extend([
        config("max_connections="),
        config("max_connections=abc"),
        config("max_connections=1.5"),
        config("max_connections=99999999999999999999"),
    ]);
    RestGroup {
        name: "connections".into(),
        pin_headers: false,
        cases,
    }
}

fn storage() -> RestGroup {
    let mut cases = Vec::new();
    // v1 matches the drive type by exact case. Anything else gets the HDD
    // effective_io_concurrency and the SSD random_page_cost.
    for version in ["12", "18"] {
        for profile in ["WEB", "DW"] {
            for disk in ["ssd", "hdd", "San", "NVME", ""] {
                cases.push(config(format!(
                    "pg_version={version}&environment_name={profile}&drive_type={disk}&cpus=8"
                )));
            }
        }
    }
    for profile in [
        "web", "Mixed", "desktop", "oltp", "dw", "", "bogus", "WEB%20",
    ] {
        cases.push(config(format!("environment_name={profile}")));
    }
    RestGroup {
        name: "storage-profile".into(),
        pin_headers: false,
        cases,
    }
}

/// io_workers multiplies the CPU count by a factor built with float64
/// additions, so some products land just above an integer.
fn aio() -> RestGroup {
    let mut cases = Vec::new();
    for profile in PROFILES {
        for disk in ["HDD", "SSD"] {
            for cpu in ["3", "7", "10", "20", "30", "40", "100"] {
                cases.push(config(format!(
                    "pg_version=18&environment_name={profile}&drive_type={disk}&cpus={cpu}&format=conf"
                )));
            }
        }
    }
    RestGroup {
        name: "aio".into(),
        pin_headers: false,
        cases,
    }
}

fn versions() -> RestGroup {
    let mut cases = Vec::new();
    // v1 parses the version as a float32 and never checks it against the
    // supported list.
    for version in [
        "17.10", "9.60", "18.0", "13.5", "12.5", "10.5", "17.9", "19", "8.4", "9", "9.35", "9.45",
        "9.55", "9.65", "1e1", "100", "0",
    ] {
        cases.push(config(format!(
            "pg_version={version}&cpus=8&drive_type=SSD"
        )));
        cases.push(config(format!(
            "pg_version={version}&cpus=8&drive_type=SSD&format=stackgres"
        )));
    }
    cases.extend([
        config("pg_version="),
        config("pg_version=abc"),
        config("pg_version=18x"),
        config("pg_version=v18"),
        config("pg_version=18.4.1"),
        config("pg_version=%2017"),
        config("pg_version=+17"),
        config("pg_version=17&pg_version=9.1"),
    ]);
    RestGroup {
        name: "versions".into(),
        pin_headers: false,
        cases,
    }
}

fn ram_and_cpu() -> RestGroup {
    let mut cases = Vec::new();
    // The v1 parser reads the leading digits and one of kb, mb, gb, tb. Any
    // other suffix means bytes.
    for ram in [
        "2048", "2gb", "2Gb", "2%20GB", "2+GB", "1.5GB", "2G", "abc", "0", "0GB", "100kb",
        "4096KB", "5000000B", "3000MB", "200GB", "1TB", "1024TB", "4096TB", "32GB", "33GB",
    ] {
        cases.push(config(format!("total_ram={ram}")));
        cases.push(config(format!(
            "total_ram={ram}&environment_name=OLTP&format=conf"
        )));
    }
    for cpu in [
        "3",
        "7",
        "64",
        "1000",
        "",
        "abc",
        "1.5",
        "99999999999999999999",
    ] {
        cases.push(config(format!("cpus={cpu}")));
        cases.push(config(format!(
            "cpus={cpu}&environment_name=DW&format=conf"
        )));
    }
    RestGroup {
        name: "ram-cpu".into(),
        pin_headers: false,
        cases,
    }
}

fn query_strings() -> RestGroup {
    let cases = vec![
        // A repeated key: the handler reads the first value, meta.arguments
        // lists them all.
        config("drive_type=SSD&drive_type=HDD"),
        config("environment_name=O%4CTP"),
        config("environment_name=OLTP&unknown=1&also%20unknown=a%26b"),
        config("format=conf&unknown=1"),
        config("&&format=conf&"),
        config("show_doc&include_pgbadger"),
        config("=empty-key&format=conf"),
        // Go drops a pair it cannot decode, and a pair with a semicolon.
        config("x=%zz&y&k=%41"),
        config("a=1;b=2&total_ram=4GB"),
        get("/v1/tuning/get-config?"),
    ];
    RestGroup {
        name: "query-strings".into(),
        pin_headers: false,
        cases,
    }
}

fn all_environments_group() -> RestGroup {
    let mut cases = Vec::new();
    for version in ["9.1", "9.4", "9.6", "13", "18"] {
        for disk in ["HDD", "SSD"] {
            cases.push(all_environments(format!(
                "pg_version={version}&drive_type={disk}"
            )));
        }
    }
    cases.extend([
        // The route ignores format, include_pgbadger, and environment_name.
        all_environments("format=conf"),
        all_environments("include_pgbadger=true"),
        all_environments("environment_name=DW&total_ram=7GB&cpus=16&max_connections=40"),
        all_environments("environment_name=bogus"),
        all_environments("pg_version=abc"),
        all_environments("os_type=freebsd"),
        all_environments("arch=ppc64"),
    ]);
    RestGroup {
        name: "all-environments".into(),
        pin_headers: false,
        cases,
    }
}

/// Every CLI case states the host facts, because their defaults come from the
/// machine the CLI runs on.
const HOST_FACTS: [&str; 9] = [
    "tune", "--ram", "8GB", "--cpus", "4", "--os", "linux", "--arch", "amd64",
];

fn tune(extra: &[&str]) -> Vec<String> {
    HOST_FACTS
        .iter()
        .chain(extra)
        .map(|arg| arg.to_string())
        .collect()
}

fn args(args: &[&str]) -> Vec<String> {
    args.iter().map(|arg| arg.to_string()).collect()
}

pub fn cli_groups() -> Vec<CliGroup> {
    vec![
        cli_formats(),
        cli_pgbadger(),
        cli_inputs(),
        cli_errors(),
        cli_version(),
    ]
}

fn cli_formats() -> CliGroup {
    let mut cases = vec![tune(&[])];
    for version in VERSIONS {
        for format in ["conf", "json", "alter_system", "stackgres"] {
            cases.push(tune(&["--version", version, "--format", format]));
        }
    }
    // The other five names, and the lowercasing of the flag value.
    for version in ["9.6", "18"] {
        for format in [
            "unix",
            "sql",
            "sg",
            "sgpostgresconfig",
            "yaml",
            "CONF",
            "Json",
            "SQL",
        ] {
            cases.push(tune(&["--version", version, "--format", format]));
        }
    }
    CliGroup {
        name: "tune-formats".into(),
        cases,
    }
}

fn cli_pgbadger() -> CliGroup {
    let mut cases = Vec::new();
    // Without --log-format the CLI picks csvlog, or jsonlog from PostgreSQL 15.
    for version in ["14", "15"] {
        for format in ["conf", "json", "sql", "sg"] {
            cases.push(tune(&[
                "--version",
                version,
                "--format",
                format,
                "--include-pgbadger",
            ]));
            for log_format in ["stderr", "csvlog", "syslog", "jsonlog", "foo"] {
                cases.push(tune(&[
                    "--version",
                    version,
                    "--format",
                    format,
                    "--include-pgbadger",
                    "--log-format",
                    log_format,
                ]));
            }
        }
    }
    cases.extend([
        tune(&["-B"]),
        tune(&["-B", "-L", "syslog"]),
        tune(&["--include-pgbadger=true", "--log-format=stderr"]),
        tune(&["--include-pgbadger=false", "--log-format", "stderr"]),
        tune(&["--log-format", "syslog"]),
    ]);
    CliGroup {
        name: "tune-pgbadger".into(),
        cases,
    }
}

fn cli_inputs() -> CliGroup {
    let mut cases = Vec::new();
    for profile in [
        "WEB", "OLTP", "DW", "MIXED", "DESKTOP", "web", "Mixed", "dw",
    ] {
        cases.push(tune(&["--profile", profile]));
    }
    for disk in ["SSD", "HDD", "SAN", "ssd", "NVME"] {
        cases.push(tune(&["--disk-type", disk]));
        cases.push(tune(&[
            "--disk-type",
            disk,
            "--profile",
            "DW",
            "--version",
            "12",
        ]));
    }
    for connections in ["1", "20", "500", "10000"] {
        cases.push(tune(&["--max-connections", connections]));
    }
    for version in ["17.10", "9.60", "18.0", "13.5", "19", "8.4", "9", "1e1"] {
        cases.push(tune(&["--version", version]));
        cases.push(tune(&["--version", version, "--format", "stackgres"]));
    }
    cases.extend([
        // Short flags and the --flag=value form.
        tune(&["-D", "HDD", "-c", "16", "-M", "250", "-F", "sql"]),
        tune(&["-DHDD", "-c16", "-M250", "-Fsql"]),
        tune(&[
            "--disk-type=SAN",
            "--cpus=32",
            "--max-connections=300",
            "--format=json",
        ]),
        tune(&["--profile=OLTP", "--version=16"]),
    ]);
    // A later flag overrides the host facts every case starts with.
    for ram in [
        "2048", "2gb", "2 GB", "1.5GB", "abc", "100kb", "1TB", "64GB",
    ] {
        cases.push(tune(&["--ram", ram]));
    }
    for cpu in ["1", "3", "64", "128"] {
        cases.push(tune(&["--cpus", cpu]));
        cases.push(tune(&[
            "--cpus",
            cpu,
            "--profile",
            "DW",
            "--disk-type",
            "HDD",
        ]));
    }
    for os in ["windows", "Windows", "unix", "darwin", "LINUX"] {
        cases.push(tune(&[
            "--os",
            os,
            "--ram",
            "64GB",
            "--max-connections",
            "5",
        ]));
        cases.push(tune(&[
            "--os",
            os,
            "--ram",
            "64GB",
            "--max-connections",
            "5",
            "--version",
            "17",
        ]));
    }
    for arch in ["386", "i686", "x86-64", "arm", "arm64"] {
        cases.push(tune(&[
            "--arch",
            arch,
            "--ram",
            "128GB",
            "--max-connections",
            "2",
        ]));
    }
    CliGroup {
        name: "tune-inputs".into(),
        cases,
    }
}

fn cli_errors() -> CliGroup {
    let cases = vec![
        // The rules reject these, and the Go CLI panics: exit code 2.
        tune(&["--os", "freebsd"]),
        tune(&["--arch", "ppc64"]),
        tune(&["--arch", "AMD64"]),
        // The flag parser rejects these: exit code 1.
        tune(&["--profile", "bogus"]),
        tune(&["--format", "bogus"]),
        tune(&["--version", "abc"]),
        tune(&["--cpus", "abc"]),
        tune(&["--cpus", "1.5"]),
        tune(&["--max-connections", "abc"]),
        tune(&["--include-pgbadger=maybe"]),
        tune(&["--no-such-flag"]),
        tune(&["--env-name", "WEB"]),
        args(&["no-such-command"]),
    ];
    CliGroup {
        name: "errors".into(),
        cases,
    }
}

fn cli_version() -> CliGroup {
    CliGroup {
        name: "version".into(),
        cases: vec![args(&["version"])],
    }
}
