---
name: "track_io_timing"
version: "9.6"
type: "boolean"
category: "Statistics / Query and Index Statistics Collector"
short_desc: "Collects timing statistics for database I/O activity."
context: "superuser"
default: "off"
url: "https://www.postgresql.org/docs/9.6/runtime-config-statistics.html#GUC-TRACK-IO-TIMING"
---

Enables timing of database I/O calls. This parameter is off by default, because it will repeatedly query the operating system for the current time, which may cause significant overhead on some platforms. You can use the [pg_test_timing](https://www.postgresql.org/docs/9.6/pgtesttiming.html) tool to measure the overhead of timing on your system. I/O timing information is displayed in [pg_stat_database](https://www.postgresql.org/docs/9.6/monitoring-stats.html#PG-STAT-DATABASE-VIEW), in the output of [EXPLAIN](https://www.postgresql.org/docs/9.6/sql-explain.html) when the `BUFFERS` option is used, and by [pg_stat_statements](https://www.postgresql.org/docs/9.6/pgstatstatements.html). Only superusers can change this setting.
