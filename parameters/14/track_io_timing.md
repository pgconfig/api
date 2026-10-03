---
name: "track_io_timing"
version: "14"
type: "boolean"
category: "Statistics / Query and Index Statistics Collector"
short_desc: "Collects timing statistics for database I/O activity."
context: "superuser"
default: "off"
url: "https://www.postgresql.org/docs/14/runtime-config-statistics.html#GUC-TRACK-IO-TIMING"
---

Enables timing of database I/O calls. This parameter is off by default, as it will repeatedly query the operating system for the current time, which may cause significant overhead on some platforms. You can use the [pg_test_timing](https://www.postgresql.org/docs/14/pgtesttiming.html) tool to measure the overhead of timing on your system. I/O timing information is displayed in [`pg_stat_database`](https://www.postgresql.org/docs/14/monitoring-stats.html#MONITORING-PG-STAT-DATABASE-VIEW), in the output of [EXPLAIN](https://www.postgresql.org/docs/14/sql-explain.html) when the `BUFFERS` option is used, by autovacuum for auto-vacuums and auto-analyzes, when [`log_autovacuum_min_duration`](https://www.postgresql.org/docs/14/runtime-config-logging.html#GUC-LOG-AUTOVACUUM-MIN-DURATION) is set and by [pg_stat_statements](https://www.postgresql.org/docs/14/pgstatstatements.html). Only superusers can change this setting.
