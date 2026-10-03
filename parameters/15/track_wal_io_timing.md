---
name: "track_wal_io_timing"
version: "15"
type: "boolean"
category: "Statistics / Cumulative Query and Index Statistics"
short_desc: "Collects timing statistics for WAL I/O activity."
context: "superuser"
default: "off"
url: "https://www.postgresql.org/docs/15/runtime-config-statistics.html#GUC-TRACK-WAL-IO-TIMING"
---

Enables timing of WAL I/O calls. This parameter is off by default, as it will repeatedly query the operating system for the current time, which may cause significant overhead on some platforms. You can use the pg_test_timing tool to measure the overhead of timing on your system. I/O timing information is displayed in [`pg_stat_wal`](https://www.postgresql.org/docs/15/monitoring-stats.html#MONITORING-PG-STAT-WAL-VIEW). Only superusers and users with the appropriate `SET` privilege can change this setting.
