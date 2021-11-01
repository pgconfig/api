package category

var (
	// PGBadgerConfig contains the extra category, that contains pgbadger's log configuration
	PGBadgerConfig = SliceOutput{
		Name:        "log_config",
		Description: "Logging configuration for pgbadger",
		Parameters: []ParamSliceOutput{
			{Format: "bool", Name: "logging_collector", Value: "on"},
			{Format: "bool", Name: "log_checkpoints", Value: "on"},
			{Format: "bool", Name: "log_connections", Value: "on"},
			{Format: "bool", Name: "log_disconnections", Value: "on"},
			{Format: "bool", Name: "log_lock_waits", Value: "on"},
			{Format: "int", Name: "log_temp_files", Value: "0"},
			{Format: "string", Name: "lc_messages", Value: "C"},
			{Format: "string", Name: "log_min_duration_statement", Value: "10s", Comment: "Adjust the minimum time to collect the data"},
			{Format: "int", Name: "log_autovacuum_min_duration", Value: "0"},
		},
	}

	// LogOptions contains an option to the log configuration
	LogOptions = map[string]SliceOutput{
		"stderr": {
			Name:        "stder_config",
			Description: "STDERR Configuration",
			Parameters: []ParamSliceOutput{
				{Format: "string", Name: "log_destination", Value: "stder"},
				{Format: "string", Name: "log_line_prefix", Value: "%t [%p]: [%l-1] user=%u,db=%d,app=%a,client=%h "},
			},
		},
		"syslog": {
			Name:        "syslog_config",
			Description: "SYSLOG Configuration",
			Parameters: []ParamSliceOutput{
				{Format: "string", Name: "log_destination", Value: "syslog"},
				{Format: "string", Name: "log_line_prefix", Value: "user=%u,db=%d,app=%a,client=%h "},
				{Format: "string", Name: "syslog_facility", Value: "LOCAL0"},
				{Format: "string", Name: "syslog_ident", Value: "postgres"},
			},
		},
		"csvlog": {
			Name:        "csv_config",
			Description: "CSV Configuration",
			Parameters: []ParamSliceOutput{
				{Format: "string", Name: "log_destination", Value: "csvlog"},
			},
		},
	}
)
