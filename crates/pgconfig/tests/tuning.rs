//! The tuning boundary: a Tuning Request goes in, and recommendations with
//! their reasons, the assumptions made, and warnings come out.

use pgconfig::{RawTuningRequest, TuningRequest, TuningResult, tune};

/// A request that supplies every fact, so the result carries no assumption.
fn complete() -> RawTuningRequest {
    RawTuningRequest {
        total_ram: Some("16GB".into()),
        total_cpu: Some(8),
        postgres_version: Some("18.4".into()),
        profile: Some("WEB".into()),
        disk_type: Some("SSD".into()),
        os: Some("linux".into()),
        arch: Some("amd64".into()),
        max_connections: Some(100),
    }
}

fn tuned(raw: RawTuningRequest) -> TuningResult {
    tune(&TuningRequest::try_from(raw).expect("a valid request"))
}

#[test]
fn a_request_is_echoed_normalized_with_a_recommendation_per_parameter() {
    let result = tuned(RawTuningRequest {
        total_ram: Some("32gb".into()),
        total_cpu: Some(16),
        postgres_version: Some(" 18.4 ".into()),
        profile: Some("oltp".into()),
        disk_type: Some("ssd".into()),
        os: Some(" Linux ".into()),
        arch: Some("X86-64".into()),
        max_connections: Some(100),
    });

    assert_eq!(
        serde_json::to_value(&result.request).unwrap(),
        serde_json::json!({
            "os": "linux",
            "arch": "amd64",
            "total_ram": "32GB",
            "profile": "OLTP",
            "disk_type": "SSD",
            "max_connections": 100,
            "total_cpu": 16,
            "postgres_version": "18.4",
        })
    );
    assert_eq!(result.recommendations["shared_buffers"].value, "8GB");
    assert_eq!(result.recommendations["max_connections"].value, "100");
    for (name, recommendation) in &result.recommendations {
        assert!(!recommendation.value.is_empty(), "{name} has no value");
        assert!(
            recommendation.reason.contains(&recommendation.value),
            "the reason of {name} does not state its value {:?}: {:?}",
            recommendation.value,
            recommendation.reason
        );
    }
}

fn reason(result: &TuningResult, name: &str) -> String {
    result.recommendations[name].reason.clone()
}

fn value(result: &TuningResult, name: &str) -> String {
    result.recommendations[name].value.clone()
}

#[test]
fn a_reason_names_the_limit_that_lowered_a_value() {
    let result = tuned(RawTuningRequest {
        os: Some("windows".into()),
        total_ram: Some("1TB".into()),
        profile: Some("OLTP".into()),
        max_connections: Some(1),
        total_cpu: Some(16),
        postgres_version: Some("17.10".into()),
        ..complete()
    });

    assert_eq!(
        reason(&result, "work_mem"),
        "Set to 2GB. Capped from 358GB to 2GB by the Windows limit of PostgreSQL releases before 18."
    );
    assert_eq!(
        reason(&result, "maintenance_work_mem"),
        "Set to 2GB. Capped from 51GB to 2GB by the Windows limit of PostgreSQL releases before 18."
    );
}

#[test]
fn limits_are_listed_in_the_order_they_applied() {
    let result = tuned(RawTuningRequest {
        arch: Some("386".into()),
        total_ram: Some("64GB".into()),
        postgres_version: Some("9.6.24".into()),
        ..complete()
    });

    assert_eq!(
        reason(&result, "shared_buffers"),
        "Set to 512MB. Capped from 16GB to 4GB for the 32-bit architecture. Then capped from 4GB to 512MB for PostgreSQL 9.6 and older."
    );
}

#[test]
fn a_value_that_equals_a_limit_without_reaching_it_is_not_reported_as_capped() {
    let cases = [
        (
            RawTuningRequest {
                arch: Some("386".into()),
                max_connections: Some(1),
                ..complete()
            },
            "work_mem",
            "4GB",
        ),
        (
            RawTuningRequest {
                arch: Some("386".into()),
                ..complete()
            },
            "shared_buffers",
            "4GB",
        ),
        (
            RawTuningRequest {
                total_ram: Some("2GB".into()),
                postgres_version: Some("9.6.24".into()),
                ..complete()
            },
            "shared_buffers",
            "512MB",
        ),
    ];

    for (request, name, expected) in cases {
        let result = tuned(request);
        assert_eq!(value(&result, name), expected, "{name}");
        assert!(
            !reason(&result, name).to_lowercase().contains("capped"),
            "{name}"
        );
    }
}

#[test]
fn a_limit_is_reported_even_when_both_values_print_the_same() {
    // 16GB and 1MB: a quarter of it is just above the 4GB limit.
    let result = tuned(RawTuningRequest {
        arch: Some("386".into()),
        total_ram: Some("16385MB".into()),
        ..complete()
    });

    assert_eq!(
        reason(&result, "shared_buffers"),
        "Set to 4GB. Capped at 4GB for the 32-bit architecture."
    );
}

#[test]
fn the_desktop_profile_explains_its_own_share_and_later_limits() {
    let modern = tuned(RawTuningRequest {
        profile: Some("desktop".into()),
        arch: Some("386".into()),
        total_ram: Some("128GB".into()),
        ..complete()
    });
    let old = tuned(RawTuningRequest {
        profile: Some("desktop".into()),
        total_ram: Some("256GB".into()),
        postgres_version: Some("9.5".into()),
        ..complete()
    });

    assert_eq!(
        reason(&modern, "shared_buffers"),
        "Set to 8GB as one sixteenth of total RAM for the DESKTOP profile."
    );
    assert_eq!(
        reason(&old, "shared_buffers"),
        "Set to 8GB as one sixteenth of total RAM for the DESKTOP profile. Then capped from 16GB to 8GB for PostgreSQL releases before 9.6."
    );
}

#[test]
fn storage_decides_io_concurrency_on_windows_too() {
    let result = tuned(RawTuningRequest {
        os: Some("windows".into()),
        ..complete()
    });

    assert_eq!(value(&result, "effective_io_concurrency"), "200");
    assert_eq!(
        reason(&result, "effective_io_concurrency"),
        "Set to 200 for the requested SSD storage; the storage policy replaces the initial Windows compatibility value."
    );
}

#[test]
fn io_workers_explains_each_bound_it_hit() {
    let workers = |total_cpu, disk_type: &str, profile: &str| {
        let result = tuned(RawTuningRequest {
            total_cpu: Some(total_cpu),
            disk_type: Some(disk_type.into()),
            profile: Some(profile.into()),
            ..complete()
        });
        (value(&result, "io_workers"), reason(&result, "io_workers"))
    };

    assert_eq!(
        workers(8, "SSD", "WEB"),
        (
            "2".into(),
            "Set to 2 from 8 logical CPUs and the WEB profile.".into()
        )
    );
    assert_eq!(
        workers(8, "HDD", "WEB"),
        (
            "3".into(),
            "Set to 3 from 8 logical CPUs and the WEB profile with the HDD storage adjustment."
                .into()
        )
    );
    assert_eq!(
        workers(1, "SSD", "WEB"),
        (
            "1".into(),
            "Set to 1 from 1 logical CPU and the WEB profile. The initial 1-worker calculation was raised to the minimum of 2. It was then capped at 1 to avoid exceeding the logical CPU count."
                .into()
        )
    );
    assert_eq!(
        workers(128, "SSD", "DW"),
        (
            "32".into(),
            "Set to 32 from 128 logical CPUs and the DW profile. The initial 52-worker calculation was capped at 32, the PostgreSQL maximum."
                .into()
        )
    );
}

#[test]
fn worker_settings_explain_their_floors() {
    let result = tuned(RawTuningRequest {
        profile: Some("DW".into()),
        total_cpu: Some(2),
        ..complete()
    });
    let large = tuned(RawTuningRequest {
        profile: Some("DW".into()),
        total_cpu: Some(32),
        ..complete()
    });

    for name in ["max_worker_processes", "max_parallel_workers"] {
        assert_eq!(
            reason(&result, name),
            "Set to 8 by applying the minimum of 8 to 2 logical CPUs."
        );
        assert_eq!(
            reason(&large, name),
            "Set to 32 to match the 32 logical CPUs."
        );
    }
    assert_eq!(
        reason(&result, "max_parallel_workers_per_gather"),
        "Set to 2 from half of 2 logical CPUs, raised to the minimum of 2 for the DW workload profile."
    );
    assert_eq!(
        reason(&large, "max_parallel_workers_per_gather"),
        "Set to 16 from half of 32 logical CPUs for the DW workload profile."
    );
}

#[test]
fn wal_buffers_explains_which_condition_set_it() {
    let wal_buffers = |total_ram: &str, profile: &str| {
        let result = tuned(RawTuningRequest {
            total_ram: Some(total_ram.into()),
            profile: Some(profile.into()),
            ..complete()
        });
        (
            value(&result, "wal_buffers"),
            reason(&result, "wal_buffers"),
        )
    };

    assert_eq!(
        wal_buffers("40GB", "OLTP"),
        (
            "32MB".into(),
            "Set to 32MB for OLTP because the profile's share of memory for shared_buffers exceeds 8GB."
                .into()
        )
    );
    assert_eq!(
        wal_buffers("16GB", "OLTP"),
        (
            "-1".into(),
            "Set to -1 to use automatic PostgreSQL tuning for the OLTP workload profile.".into()
        )
    );
    assert_eq!(
        wal_buffers("16GB", "DW"),
        (
            "64MB".into(),
            "Set to 64MB for write-heavy DW workloads.".into()
        )
    );
}

#[test]
fn settings_of_old_releases_explain_themselves() {
    let result = tuned(RawTuningRequest {
        postgres_version: Some("9.4.26".into()),
        ..complete()
    });

    assert_eq!(
        reason(&result, "checkpoint_segments"),
        "Set to 16 as the legacy checkpoint segment count for PostgreSQL releases before 9.5."
    );
    assert!(!result.recommendations.contains_key("min_wal_size"));
    assert!(!result.recommendations.contains_key("io_workers"));
}

#[test]
fn listen_addresses_is_never_recommended() {
    for major in pgconfig::PgMajor::supported() {
        let result = tuned(RawTuningRequest {
            postgres_version: Some(major.to_string()),
            ..complete()
        });

        assert!(
            !result.recommendations.contains_key("listen_addresses"),
            "{major}"
        );
    }
}

#[test]
fn the_same_request_gives_the_same_result() {
    let first = tuned(complete());
    let second = tuned(complete());

    assert_eq!(first, second);
    assert_eq!(
        serde_json::to_string(&first).unwrap(),
        serde_json::to_string(&second).unwrap()
    );
}

/// A request with only the three required facts.
fn required_only() -> RawTuningRequest {
    RawTuningRequest {
        total_ram: Some("8GB".into()),
        total_cpu: Some(4),
        postgres_version: Some("17.10".into()),
        ..RawTuningRequest::default()
    }
}

#[test]
fn every_default_is_reported_as_an_assumption() {
    let result = tuned(required_only());

    assert_eq!(
        serde_json::to_value(&result.assumptions).unwrap(),
        serde_json::json!([
            {"field": "profile", "value": "WEB", "message": "profile was not supplied. Assumed WEB."},
            {"field": "disk_type", "value": "SSD", "message": "disk_type was not supplied. Assumed SSD."},
            {"field": "os", "value": "linux", "message": "os was not supplied. Assumed linux."},
            {"field": "arch", "value": "amd64", "message": "arch was not supplied. Assumed amd64."},
            {"field": "max_connections", "value": "100", "message": "max_connections was not supplied. Assumed 100."},
        ])
    );
    assert_eq!(
        serde_json::to_value(&result.request).unwrap(),
        serde_json::json!({
            "os": "linux",
            "arch": "amd64",
            "total_ram": "8GB",
            "profile": "WEB",
            "disk_type": "SSD",
            "max_connections": 100,
            "total_cpu": 4,
            "postgres_version": "17.10",
        })
    );
}

#[test]
fn a_complete_request_has_no_assumptions() {
    assert!(tuned(complete()).assumptions.is_empty());
}

#[test]
fn only_the_omitted_facts_are_assumed() {
    let result = tuned(RawTuningRequest {
        profile: Some("DW".into()),
        ..required_only()
    });

    let fields: Vec<&str> = result.assumptions.iter().map(|a| a.field).collect();
    assert_eq!(fields, ["disk_type", "os", "arch", "max_connections"]);
}

#[test]
fn an_unusually_high_connection_count_is_a_warning_not_an_error() {
    let result = tuned(RawTuningRequest {
        max_connections: Some(5000),
        ..complete()
    });

    assert_eq!(
        serde_json::to_value(&result.warnings).unwrap(),
        serde_json::json!([{
            "code": "high_max_connections",
            "message": "max_connections is 5000. Every connection takes memory, and work_mem is divided among them. Above 1000 connections, consider a connection pooler in front of PostgreSQL.",
        }])
    );
    assert_eq!(value(&result, "max_connections"), "5000");
}

#[test]
fn an_ordinary_request_has_no_warnings() {
    assert!(tuned(complete()).warnings.is_empty());
    assert!(
        tuned(RawTuningRequest {
            max_connections: Some(1000),
            ..complete()
        })
        .warnings
        .is_empty()
    );
}

fn error(raw: RawTuningRequest) -> pgconfig::TuningError {
    TuningRequest::try_from(raw).expect_err("an invalid request")
}

#[test]
fn every_missing_and_invalid_fact_is_reported_at_once() {
    let error = error(RawTuningRequest {
        total_ram: Some("16".into()),
        ..RawTuningRequest::default()
    });

    assert_eq!(error.missing_fields(), ["total_cpu", "postgres_version"]);
    assert_eq!(
        error.to_string(),
        "Invalid tuning request. total_cpu is required: the number of logical CPUs, such as 8. postgres_version is required: the PostgreSQL version, such as 18.4. total_ram: \"16\" has no unit. Use a positive integer followed by B, KB, MB, GB, or TB, such as 16GB or 1536MB."
    );
}

#[test]
fn an_empty_request_names_the_three_required_facts() {
    let error = error(RawTuningRequest::default());

    assert_eq!(
        error.missing_fields(),
        ["total_ram", "total_cpu", "postgres_version"]
    );
}

#[test]
fn invalid_optional_facts_are_errors_not_defaults() {
    let cases = [
        (
            RawTuningRequest {
                profile: Some("bogus".into()),
                ..complete()
            },
            "profile",
        ),
        (
            RawTuningRequest {
                disk_type: Some("nvme".into()),
                ..complete()
            },
            "disk_type",
        ),
        (
            RawTuningRequest {
                os: Some("freebsd".into()),
                ..complete()
            },
            "os",
        ),
        (
            RawTuningRequest {
                arch: Some("ppc64".into()),
                ..complete()
            },
            "arch",
        ),
        (
            RawTuningRequest {
                max_connections: Some(0),
                ..complete()
            },
            "max_connections",
        ),
        (
            RawTuningRequest {
                max_connections: Some(-5),
                ..complete()
            },
            "max_connections",
        ),
        (
            RawTuningRequest {
                total_cpu: Some(0),
                ..complete()
            },
            "total_cpu",
        ),
        (
            RawTuningRequest {
                total_ram: Some("1.5GB".into()),
                ..complete()
            },
            "total_ram",
        ),
        (
            RawTuningRequest {
                postgres_version: Some("v18.4".into()),
                ..complete()
            },
            "postgres_version",
        ),
        (
            RawTuningRequest {
                postgres_version: Some("19".into()),
                ..complete()
            },
            "postgres_version",
        ),
    ];

    for (request, field) in cases {
        let error = error(request);
        let fields: Vec<&str> = error.problems.iter().map(|problem| problem.field).collect();
        assert_eq!(fields, [field]);
        assert!(error.missing_fields().is_empty(), "{field}");
        assert!(error.to_string().contains(&format!("{field}: ")), "{error}");
    }
}

#[test]
fn a_large_connection_count_is_accepted() {
    let result = tuned(RawTuningRequest {
        max_connections: Some(250_000),
        ..complete()
    });

    assert_eq!(result.request.max_connections, 250_000);
}

/// Full results, so a change to any value or reason shows up in review.
#[test]
fn full_results_match_their_snapshots() {
    insta::assert_json_snapshot!(
        "oltp_on_postgresql_18",
        tuned(RawTuningRequest {
            total_ram: Some("64GB".into()),
            total_cpu: Some(32),
            postgres_version: Some("18.4".into()),
            profile: Some("OLTP".into()),
            ..complete()
        })
    );
    insta::assert_json_snapshot!("defaults_on_postgresql_17", tuned(required_only()));
    insta::assert_json_snapshot!(
        "windows_32_bit_on_postgresql_9_4",
        tuned(RawTuningRequest {
            total_ram: Some("64GB".into()),
            postgres_version: Some("9.4.26".into()),
            os: Some("windows".into()),
            arch: Some("i686".into()),
            disk_type: Some("HDD".into()),
            max_connections: Some(2000),
            ..complete()
        })
    );
}
