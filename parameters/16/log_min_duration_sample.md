---
name: "log_min_duration_sample"
version: "16"
type: "integer"
category: "Reporting and Logging / When to Log"
short_desc: "Sets the minimum execution time above which a sample of statements will be logged. Sampling is determined by log_statement_sample_rate."
extra_desc: "Zero logs a sample of all queries. -1 turns this feature off."
context: "superuser"
unit: "ms"
default: "-1"
min: "-1"
max: "2147483647"
url: "https://www.postgresql.org/docs/16/runtime-config-logging.html#GUC-LOG-MIN-DURATION-SAMPLE"
---

Allows sampling the duration of completed statements that ran for at least the specified amount of time. This produces the same kind of log entries as [`log_min_duration_statement`](https://www.postgresql.org/docs/16/runtime-config-logging.html#GUC-LOG-MIN-DURATION-STATEMENT), but only for a subset of the executed statements, with sample rate controlled by [`log_statement_sample_rate`](https://www.postgresql.org/docs/16/runtime-config-logging.html#GUC-LOG-STATEMENT-SAMPLE-RATE). For example, if you set it to `100ms` then all SQL statements that run 100ms or longer will be considered for sampling. Enabling this parameter can be helpful when the traffic is too high to log all queries. If this value is specified without units, it is taken as milliseconds. Setting this to zero samples all statement durations. `-1` (the default) disables sampling statement durations. Only superusers and users with the appropriate `SET` privilege can change this setting.

This setting has lower priority than `log_min_duration_statement`, meaning that statements with durations exceeding `log_min_duration_statement` are not subject to sampling and are always logged.

Other notes for `log_min_duration_statement` apply also to this setting.
