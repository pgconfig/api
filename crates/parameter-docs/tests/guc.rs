//! The GUC tables in C read into settings.

use std::collections::BTreeMap;

use pgconfig_parameter_docs::guc::{Setting, Sources, settings};

#[test]
fn postgresql_19_data_preserves_types_macros_units_and_build_conditions() {
    let tables = format!(
        "{GROUPS}\nconst struct config_enum_entry methods[] = {{\n{{\"worker\", 1, false}}, {{\"hidden\", 2, true}}, {{NULL, 0, false}}\n}};"
    );
    let source = Sources {
        tables: &tables,
        header: "",
        options: &[],
        headers: &["#define MAX_IO_WORKERS 32\n"],
        release: "19beta4",
    };
    let data = r#"[
      # Data, not executable Perl.
      { name => 'io_max_workers', type => 'int', context => 'PGC_SIGHUP', group => 'RESOURCES_MEM',
        short_desc => 'Worker\'s maximum.', variable => 'workers', boot_val => '8', min => '1', max => 'MAX_IO_WORKERS' },
      { name => 'io_worker_idle_timeout', type => 'int', context => 'PGC_SIGHUP', group => 'RESOURCES_MEM',
        short_desc => 'Idle time.', variable => 'timeout', boot_val => '60000', min => '0', max => 'INT_MAX', flags => 'GUC_UNIT_MS' },
      { name => 'io_method', type => 'enum', context => 'PGC_POSTMASTER', group => 'RESOURCES_MEM',
        short_desc => 'Method.', variable => 'method', boot_val => '1', options => 'methods' },
      { name => 'jit', type => 'bool', context => 'PGC_USERSET', group => 'RESOURCES_MEM',
        short_desc => 'JIT.', variable => 'jit', boot_val => 'false' },
      { name => 'server_version', type => 'string', context => 'PGC_INTERNAL', group => 'UNGROUPED',
        short_desc => 'Version.', variable => 'version', boot_val => 'PG_VERSION' },
      { name => 'weight', type => 'real', context => 'PGC_SIGHUP', group => 'RESOURCES_MEM',
        short_desc => 'Weight.', variable => 'weight', boot_val => '1.0', min => '0.0', max => '10.0' },
      { name => 'debug', type => 'bool', context => 'PGC_USERSET', group => 'UNGROUPED',
        short_desc => 'Debug.', variable => 'debug', boot_val => 'true', ifdef => 'USE_ASSERT_CHECKING' },
    ]"#;
    let settings = pgconfig_parameter_docs::guc::settings_from_dat(&source, data).unwrap();
    assert_eq!(settings["io_max_workers"].default.as_deref(), Some("8"));
    assert_eq!(settings["io_max_workers"].max.as_deref(), Some("32"));
    assert_eq!(settings["io_max_workers"].short_desc, "Worker's maximum.");
    assert_eq!(
        settings["io_worker_idle_timeout"].unit.as_deref(),
        Some("ms")
    );
    assert_eq!(settings["io_method"].values, ["worker"]);
    assert_eq!(settings["io_method"].context, "postmaster");
    assert_eq!(settings["jit"].default.as_deref(), Some("off"));
    assert_eq!(
        settings["server_version"].default.as_deref(),
        Some("19beta4")
    );
    assert_eq!(settings["weight"].max.as_deref(), Some("10"));
    assert!(!settings.contains_key("debug"));
}

/// Group names written the way PostgreSQL 16 and later write them.
const GROUPS: &str = r#"
const char *const config_group_names[] =
{
	[UNGROUPED] = gettext_noop("Ungrouped"),
	[RESOURCES_MEM] = gettext_noop("Resource Usage / Memory"),
	[WAL_SETTINGS] = gettext_noop("Write-Ahead Log / Settings"),
};
"#;

fn read(tables: &str) -> BTreeMap<String, Setting> {
    read_with(tables, &[], &[])
}

fn read_with(tables: &str, options: &[&str], headers: &[&str]) -> BTreeMap<String, Setting> {
    let tables = format!("{GROUPS}\n{tables}");
    settings(&Sources {
        tables: &tables,
        header: "",
        options,
        headers,
        release: "18.6",
    })
    .unwrap()
}

#[test]
fn an_integer_parameter_has_its_context_category_unit_and_limits() {
    let tables = r#"
struct config_int ConfigureNamesInt[] =
{
	{
		{"work_mem", PGC_USERSET, RESOURCES_MEM,
			gettext_noop("Sets the maximum memory to be used for query workspaces."),
			gettext_noop("This much memory can be used by each internal "
						 "sort operation and hash table before switching to "
						 "temporary disk files."),
			GUC_UNIT_KB | GUC_EXPLAIN
		},
		&work_mem,
		4096, 64, 2147483647,
		NULL, NULL, NULL
	},

	/* End-of-list marker */
	{
		{NULL, 0, 0, NULL, NULL}, NULL, 0, 0, 0, NULL, NULL, NULL
	}
};
"#;

    assert_eq!(
        read(tables)["work_mem"],
        Setting {
            vartype: "integer".into(),
            category: "Resource Usage / Memory".into(),
            short_desc: "Sets the maximum memory to be used for query workspaces.".into(),
            extra_desc: Some(
                "This much memory can be used by each internal sort operation and hash table before switching to temporary disk files."
                    .into()
            ),
            context: "user".into(),
            unit: Some("kB".into()),
            default: Some("4096".into()),
            min: Some("64".into()),
            max: Some("2147483647".into()),
            values: vec![],
        }
    );
}

fn integer(name: &str, flags: &str, values: &str) -> String {
    format!(
        r#"
	{{
		{{"{name}", PGC_USERSET, RESOURCES_MEM,
			gettext_noop("Short."),
			NULL,
			{flags}
		}},
		&variable,
		{values},
		NULL, NULL, NULL
	}},"#
    )
}

fn integers(entries: &[String]) -> String {
    format!(
        "struct config_int ConfigureNamesInt[] =\n{{{}\n\t{{\n\t\t{{NULL, 0, 0, NULL, NULL}}, NULL, 0, 0, 0, NULL, NULL, NULL\n\t}}\n}};\n",
        entries.concat()
    )
}

#[test]
fn limits_written_as_macros_take_the_values_of_a_64_bit_linux_build() {
    let tables = integers(&[
        integer("max_connections", "0", "100, 1, MAX_BACKENDS"),
        integer(
            "temp_buffers",
            "GUC_UNIT_BLOCKS | GUC_EXPLAIN",
            "1024, 100, INT_MAX / 2",
        ),
        integer("work_mem", "GUC_UNIT_KB", "4096, 64, MAX_KILOBYTES"),
    ]);

    let settings = read_with(&tables, &[], &["#define MAX_BACKENDS\t0x3FFFF"]);

    let limits = |name: &str| {
        let setting = &settings[name];
        (setting.unit.clone(), setting.max.clone())
    };
    assert_eq!(limits("max_connections"), (None, Some("262143".into())));
    assert_eq!(
        limits("temp_buffers"),
        (Some("8kB".into()), Some("1073741823".into()))
    );
    assert_eq!(
        limits("work_mem"),
        (Some("kB".into()), Some("2147483647".into()))
    );
}

#[test]
fn a_macro_nothing_defines_stops_the_extraction() {
    let tables = format!(
        "{GROUPS}\n{}",
        integers(&[integer("work_mem", "0", "4096, 64, UNHEARD_OF")])
    );

    let error = settings(&Sources {
        tables: &tables,
        header: "",
        options: &[],
        headers: &[],
        release: "18.6",
    })
    .unwrap_err();

    assert_eq!(
        error,
        "work_mem: UNHEARD_OF is not defined. Add it to the platform table"
    );
}

#[test]
fn booleans_reals_and_strings_read_as_pg_settings_shows_them() {
    let tables = r#"
static struct config_bool ConfigureNamesBool[] =
{
	{
		{"fsync", PGC_SIGHUP, WAL_SETTINGS,
			gettext_noop("Forces synchronization of updates to disk."),
			NULL
		},
		&enableFsync,
		true,
		NULL, NULL, NULL
	},
	{
		{"zero_damaged_pages", PGC_SUSET, WAL_SETTINGS,
			gettext_noop("Continues processing past damaged page headers."),
			NULL
		},
		&zero_damaged_pages,
		false,
		NULL, NULL, NULL
	},
	{
		{NULL, 0, 0, NULL, NULL}, NULL, false, NULL, NULL, NULL
	}
};

static struct config_real ConfigureNamesReal[] =
{
	{
		{"checkpoint_completion_target", PGC_SIGHUP, WAL_SETTINGS,
			gettext_noop("Time spent flushing dirty buffers during checkpoint, as fraction of checkpoint interval."),
			NULL
		},
		&CheckPointCompletionTarget,
		0.9, 0.0, 1.0,
		NULL, NULL, NULL
	},
	{
		{"seq_page_cost", PGC_USERSET, WAL_SETTINGS,
			gettext_noop("Sets the planner's estimate of the cost of a sequentially fetched disk page."),
			NULL,
			GUC_EXPLAIN
		},
		&seq_page_cost,
		DEFAULT_SEQ_PAGE_COST, 0, DBL_MAX,
		NULL, NULL, NULL
	},
	{
		{NULL, 0, 0, NULL, NULL}, NULL, 0.0, 0.0, 0.0, NULL, NULL, NULL
	}
};

static struct config_string ConfigureNamesString[] =
{
	{
		{"unix_socket_directories", PGC_POSTMASTER, WAL_SETTINGS,
			gettext_noop("Sets the directories where Unix-domain sockets will be created."),
			NULL,
			GUC_LIST_INPUT | GUC_LIST_QUOTE | GUC_SUPERUSER_ONLY
		},
		&Unix_socket_directories,
		DEFAULT_PGSOCKET_DIR,
		NULL, NULL, NULL
	},
	{
		{"search_path", PGC_USERSET, WAL_SETTINGS,
			gettext_noop("Sets the schema search order for names that are not schema-qualified."),
			NULL,
			GUC_LIST_INPUT | GUC_LIST_QUOTE | GUC_EXPLAIN
		},
		&namespace_search_path,
		"\"$user\", public",
		check_search_path, assign_search_path, NULL
	},
	{
		{"data_directory", PGC_POSTMASTER, WAL_SETTINGS,
			gettext_noop("Sets the server's data directory."),
			NULL,
			GUC_SUPERUSER_ONLY | GUC_DISALLOW_IN_AUTO_FILE
		},
		&data_directory,
		NULL,
		NULL, NULL, NULL
	},
	{
		{NULL, 0, 0, NULL, NULL}, NULL, NULL, NULL, NULL, NULL
	}
};
"#;

    let settings = read_with(
        tables,
        &[],
        &[
            "#define DEFAULT_SEQ_PAGE_COST  1.0",
            "#define DEFAULT_PGSOCKET_DIR  \"/tmp\"",
        ],
    );

    let values = |name: &str| {
        let setting = &settings[name];
        (
            setting.vartype.as_str(),
            setting.default.as_deref(),
            setting.min.as_deref(),
            setting.max.as_deref(),
        )
    };
    assert_eq!(values("fsync"), ("boolean", Some("on"), None, None));
    assert_eq!(
        values("zero_damaged_pages"),
        ("boolean", Some("off"), None, None)
    );
    assert_eq!(
        values("checkpoint_completion_target"),
        ("floating point", Some("0.9"), Some("0"), Some("1"))
    );
    assert_eq!(
        values("seq_page_cost"),
        ("floating point", Some("1"), Some("0"), Some("1.79769e+308"))
    );
    assert_eq!(
        values("unix_socket_directories"),
        ("string", Some("/tmp"), None, None)
    );
    assert_eq!(
        values("search_path"),
        ("string", Some("\"$user\", public"), None, None)
    );
    assert_eq!(values("data_directory"), ("string", None, None, None));
}

#[test]
fn an_enum_lists_the_values_a_linux_build_accepts_and_names_its_default() {
    let tables = r#"
static const struct config_enum_entry constraint_exclusion_options[] = {
	{"partition", CONSTRAINT_EXCLUSION_PARTITION, false},
	{"on", CONSTRAINT_EXCLUSION_ON, false},
	{"off", CONSTRAINT_EXCLUSION_OFF, false},
	{"true", CONSTRAINT_EXCLUSION_ON, true},
	{NULL, 0, false}
};

extern const struct config_enum_entry sync_method_options[];

static struct config_enum ConfigureNamesEnum[] =
{
	{
		{"constraint_exclusion", PGC_USERSET, WAL_SETTINGS,
			gettext_noop("Enables the planner to use constraints to optimize queries."),
			NULL,
			GUC_EXPLAIN
		},
		&constraint_exclusion,
		CONSTRAINT_EXCLUSION_PARTITION, constraint_exclusion_options,
		NULL, NULL, NULL
	},
	{
		{"wal_sync_method", PGC_SIGHUP, WAL_SETTINGS,
			gettext_noop("Selects the method used for forcing WAL updates to disk."),
			NULL
		},
		&sync_method,
		DEFAULT_SYNC_METHOD, sync_method_options,
		NULL, assign_xlog_sync_method, NULL
	},
	{
		{NULL, 0, 0, NULL, NULL}, NULL, 0, NULL, NULL, NULL, NULL
	}
};
"#;
    // xlog.c, where the options of wal_sync_method live.
    let xlog = r#"
const struct config_enum_entry sync_method_options[] = {
	{"fsync", SYNC_METHOD_FSYNC, false},
#ifdef HAVE_FSYNC_WRITETHROUGH
	{"fsync_writethrough", SYNC_METHOD_FSYNC_WRITETHROUGH, false},
#endif
#ifdef HAVE_FDATASYNC
	{"fdatasync", SYNC_METHOD_FDATASYNC, false},
#endif
	{"open_datasync", SYNC_METHOD_OPEN_DSYNC, false},
	{NULL, 0, false}
};
"#;

    let settings = read_with(tables, &[xlog], &[]);

    let values = |name: &str| {
        (
            settings[name].default.clone(),
            settings[name].values.clone(),
        )
    };
    assert_eq!(
        values("constraint_exclusion"),
        (
            Some("partition".into()),
            vec!["partition".into(), "on".into(), "off".into()]
        )
    );
    assert_eq!(
        values("wal_sync_method"),
        (
            Some("fdatasync".into()),
            vec!["fsync".into(), "fdatasync".into(), "open_datasync".into()]
        )
    );
}

#[test]
fn before_postgresql_16_the_group_names_follow_the_order_of_the_header() {
    let tables = r#"
const char *const config_group_names[] =
{
	/* UNGROUPED */
	gettext_noop("Ungrouped"),
	/* FILE_LOCATIONS */
	gettext_noop("File Locations"),
	/* RESOURCES_MEM */
	gettext_noop("Resource Usage / Memory"),
	/* help_config wants this */
	NULL
};

static struct config_int ConfigureNamesInt[] =
{
	{
		{"work_mem", PGC_USERSET, RESOURCES_MEM,
			gettext_noop("Sets the maximum memory to be used for query workspaces."),
			NULL,
			GUC_UNIT_KB
		},
		&work_mem,
		1024, 64, 2147483647,
		NULL, NULL, NULL
	},
	{
		{NULL, 0, 0, NULL, NULL}, NULL, 0, 0, 0, NULL, NULL, NULL
	}
};
"#;
    let header = "enum config_group\n{\n\tUNGROUPED,\n\tFILE_LOCATIONS,\n\tRESOURCES_MEM\n};\n";

    let settings = settings(&Sources {
        tables,
        header,
        options: &[],
        headers: &[],
        release: "18.6",
    })
    .unwrap();

    assert_eq!(settings["work_mem"].category, "Resource Usage / Memory");
}

#[test]
fn a_parameter_a_standard_build_leaves_out_is_absent() {
    let tables = r#"
struct config_bool ConfigureNamesBool[] =
{
	{
		{"debug_assertions", PGC_INTERNAL, WAL_SETTINGS,
			gettext_noop("Shows whether the running server has assertion checks enabled."),
			NULL,
			GUC_NOT_IN_SAMPLE
		},
		&assert_enabled,
#ifdef USE_ASSERT_CHECKING
		true,
#else
		false,
#endif
		NULL, NULL, NULL
	},
#ifdef LOCK_DEBUG
	{
		{"trace_locks", PGC_SUSET, WAL_SETTINGS,
			gettext_noop("Emits information about lock usage."),
			NULL,
			GUC_NOT_IN_SAMPLE
		},
		&Trace_locks,
		false,
		NULL, NULL, NULL
	},
#endif
	{
		{NULL, 0, 0, NULL, NULL}, NULL, false, NULL, NULL, NULL
	}
};
"#;

    let settings = read(tables);

    assert_eq!(settings["debug_assertions"].default.as_deref(), Some("off"));
    assert!(!settings.contains_key("trace_locks"));
}

#[test]
fn a_condition_the_platform_table_does_not_decide_stops_the_extraction() {
    let tables = format!(
        "{GROUPS}\n{}",
        integers(&[format!(
            "\n#ifdef SOMETHING_NEW{}\n#endif",
            integer("x", "0", "1, 0, 2")
        )])
    );

    let error = settings(&Sources {
        tables: &tables,
        header: "",
        options: &[],
        headers: &[],
        release: "18.6",
    })
    .unwrap_err();

    assert_eq!(
        error,
        "ConfigureNamesInt: the preprocessor condition SOMETHING_NEW is not in the platform table. Add whether a standard 64-bit Linux build defines it"
    );
}

#[test]
fn a_macro_defined_differently_by_platform_takes_the_linux_definition() {
    let tables = integers(&[integer(
        "effective_io_concurrency",
        "0",
        "DEFAULT_EFFECTIVE_IO_CONCURRENCY, 0, MAX_IO_CONCURRENCY",
    )]);
    let bufmgr = "
#define MAX_IO_CONCURRENCY 1000

/* Only some platforms support prefetching. */
#ifdef USE_PREFETCH
#define DEFAULT_EFFECTIVE_IO_CONCURRENCY 16
#else
#define DEFAULT_EFFECTIVE_IO_CONCURRENCY 0
#endif
";

    let settings = read_with(&tables, &[], &[bufmgr]);

    let setting = &settings["effective_io_concurrency"];
    assert_eq!(
        (setting.default.as_deref(), setting.max.as_deref()),
        (Some("16"), Some("1000"))
    );
}

#[test]
fn an_options_array_with_an_offset_skips_its_first_entries() {
    let tables = r#"
static const struct config_enum_entry ssl_protocol_versions_info[] = {
	{"", PG_TLS_ANY, false},
	{"TLSv1.2", PG_TLS1_2_VERSION, false},
	{"TLSv1.3", PG_TLS1_3_VERSION, false},
	{NULL, 0, false}
};

struct config_enum ConfigureNamesEnum[] =
{
	{
		{"ssl_min_protocol_version", PGC_SIGHUP, WAL_SETTINGS,
			gettext_noop("Sets the minimum SSL/TLS protocol version to use."),
			NULL
		},
		&ssl_min_protocol_version,
		PG_TLS1_2_VERSION,
		ssl_protocol_versions_info + 1, /* don't allow PG_TLS_ANY */
		NULL, NULL, NULL
	},
	{
		{NULL, 0, 0, NULL, NULL}, NULL, 0, NULL, NULL, NULL, NULL
	}
};
"#;

    let settings = read(tables);

    assert_eq!(
        settings["ssl_min_protocol_version"].values,
        ["TLSv1.2", "TLSv1.3"]
    );
}

#[test]
fn a_definition_a_linux_build_skips_is_not_used() {
    let tables = integers(&[integer(
        "io_combine_limit",
        "0",
        "16, 1, MAX_IO_COMBINE_LIMIT",
    )]);
    // The fallback applies only where <limits.h> lacks IOV_MAX, and Linux
    // has it.
    let pg_iovec = "
#ifndef IOV_MAX
#define IOV_MAX 16
#endif
#define MAX_IO_COMBINE_LIMIT Min(IOV_MAX, 128)
";
    let debug = "
#ifdef LOCK_DEBUG
#define DEBUG_ONLY_LIMIT 5
#endif
";

    let settings = read_with(&tables, &[], &[pg_iovec]);
    let error = settings_error(
        &integers(&[integer("trace_lock_table", "0", "0, 0, DEBUG_ONLY_LIMIT")]),
        debug,
    );

    assert_eq!(settings["io_combine_limit"].max.as_deref(), Some("128"));
    assert_eq!(
        error,
        "trace_lock_table: DEBUG_ONLY_LIMIT is not defined. Add it to the platform table"
    );
}

fn settings_error(tables: &str, header: &str) -> String {
    let tables = format!("{GROUPS}\n{tables}");
    settings(&Sources {
        tables: &tables,
        header: "",
        options: &[],
        headers: &[header],
        release: "18.6",
    })
    .unwrap_err()
}

#[test]
fn a_unit_flag_the_extractor_does_not_know_stops_the_extraction() {
    let tables = integers(&[integer("future_limit", "GUC_UNIT_LIGHTYEAR", "1, 0, 2")]);

    let error = settings_error(&tables, "");

    assert_eq!(
        error,
        "future_limit: GUC_UNIT_LIGHTYEAR is a unit the extractor does not know. Add the unit pg_settings shows for it to unit() in src/guc.rs"
    );
}
