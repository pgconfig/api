---
name: "track_cost_delay_timing"
version: "19"
type: "boolean"
category: "Statistics / Cumulative Query and Index Statistics"
short_desc: "Collects timing statistics for cost-based vacuum delay."
context: "superuser"
default: "off"
url: "https://www.postgresql.org/docs/19/runtime-config-statistics.html#GUC-TRACK-COST-DELAY-TIMING"
---

Enables timing of cost-based vacuum delay (see [Cost-based Vacuum Delay](https://www.postgresql.org/docs/19/runtime-config-vacuum.html#RUNTIME-CONFIG-RESOURCE-VACUUM-COST)). This parameter is off by default, as it will repeatedly query the operating system for the current time, which may cause significant overhead on some platforms. You can use the [pg_test_timing](https://www.postgresql.org/docs/19/pgtesttiming.html) tool to measure the overhead of timing on your system. Cost-based vacuum delay timing information is displayed in [`pg_stat_progress_vacuum`](https://www.postgresql.org/docs/19/progress-reporting.html#VACUUM-PROGRESS-REPORTING), [`pg_stat_progress_analyze`](https://www.postgresql.org/docs/19/progress-reporting.html#ANALYZE-PROGRESS-REPORTING), in the output of [VACUUM](https://www.postgresql.org/docs/19/sql-vacuum.html) and [ANALYZE](https://www.postgresql.org/docs/19/sql-analyze.html) when the `VERBOSE` option is used, and by autovacuum for auto-vacuums and auto-analyzes when [`log_autovacuum_min_duration`](https://www.postgresql.org/docs/19/runtime-config-logging.html#GUC-LOG-AUTOVACUUM-MIN-DURATION) is set. Only superusers and users with the appropriate `SET` privilege can change this setting.
