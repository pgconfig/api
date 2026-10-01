//! Replays the CLI goldens recorded from the Go `pgconfigctl` against this
//! binary.

use std::path::Path;

use pgconfig_golden::{check_cli, golden_dir};

#[test]
fn the_cli_reproduces_every_golden() {
    let report = check_cli(&golden_dir(), Path::new(env!("CARGO_BIN_EXE_pgconfigctl"))).unwrap();

    assert!(
        report.checked > 200,
        "only {} records were checked",
        report.checked
    );
    assert!(report.passed(), "{}", report.summary());
}
