//! A parameter written as a file of `parameters/`.

use pgconfig_parameter_docs::corpus::{Parameter, render};
use pgconfig_parameter_docs::guc::Setting;

#[test]
fn a_file_is_front_matter_then_the_manual_text() {
    let parameter = Parameter {
        name: "wal_level".into(),
        version: "18".into(),
        param_type: Some("enum".into()),
        url: "https://www.postgresql.org/docs/18/runtime-config-wal.html#GUC-WAL-LEVEL".into(),
        text: "Determines how much information is written to the WAL.".into(),
        setting: Some(Setting {
            vartype: "enum".into(),
            category: "Write-Ahead Log / Settings".into(),
            short_desc: "Sets the level of information written to the WAL.".into(),
            extra_desc: Some("A \"quoted\" word.".into()),
            context: "postmaster".into(),
            unit: None,
            default: Some("replica".into()),
            min: None,
            max: None,
            values: vec!["minimal".into(), "replica".into(), "logical".into()],
        }),
    };

    assert_eq!(
        render(&parameter),
        r#"---
name: "wal_level"
version: "18"
type: "enum"
category: "Write-Ahead Log / Settings"
short_desc: "Sets the level of information written to the WAL."
extra_desc: "A \"quoted\" word."
context: "postmaster"
default: "replica"
values: ["minimal", "replica", "logical"]
url: "https://www.postgresql.org/docs/18/runtime-config-wal.html#GUC-WAL-LEVEL"
---

Determines how much information is written to the WAL.
"#
    );
}

#[test]
fn a_parameter_a_standard_build_lacks_has_only_what_the_manual_says() {
    let parameter = Parameter {
        name: "trace_locks".into(),
        version: "18".into(),
        param_type: Some("boolean".into()),
        url: "https://www.postgresql.org/docs/18/runtime-config-developer.html#GUC-TRACE-LOCKS"
            .into(),
        text: "If on, emit information about lock usage.".into(),
        setting: None,
    };

    assert_eq!(
        render(&parameter),
        r#"---
name: "trace_locks"
version: "18"
type: "boolean"
url: "https://www.postgresql.org/docs/18/runtime-config-developer.html#GUC-TRACE-LOCKS"
---

If on, emit information about lock usage.
"#
    );
}

#[test]
fn the_type_comes_from_the_guc_tables_when_the_build_has_the_parameter() {
    // The manual writes the SQL type of some parameters, such as pg_lsn.
    let parameter = Parameter {
        name: "recovery_target_lsn".into(),
        version: "18".into(),
        param_type: Some("pg_lsn".into()),
        url: "https://www.postgresql.org/docs/18/runtime-config-wal.html#GUC-RECOVERY-TARGET-LSN"
            .into(),
        text: "This parameter specifies the LSN of the write-ahead log location up to which recovery will proceed.".into(),
        setting: Some(Setting {
            vartype: "string".into(),
            category: "Write-Ahead Log / Recovery Target".into(),
            short_desc: "Sets the LSN of the write-ahead log location up to which recovery will proceed.".into(),
            extra_desc: None,
            context: "postmaster".into(),
            unit: None,
            default: Some(String::new()),
            min: None,
            max: None,
            values: vec![],
        }),
    };

    let file = render(&parameter);

    assert!(file.contains("\ntype: \"string\"\n"), "{file}");
}
