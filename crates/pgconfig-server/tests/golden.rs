//! Replays the REST goldens recorded from the Go API against this binary.

use std::path::Path;

use pgconfig_golden::{check_server, golden_dir};

#[test]
fn the_server_reproduces_every_golden() {
    let report = check_server(
        &golden_dir(),
        Path::new(env!("CARGO_BIN_EXE_pgconfig-server")),
    )
    .unwrap();

    assert!(
        report.checked > 5900,
        "only {} records were checked",
        report.checked
    );
    assert!(report.passed(), "{}", report.summary());
}
