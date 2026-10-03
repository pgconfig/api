---
name: "track_wal_io_timing"
version: "18"
type: "boolean"
category: "Statistics / Cumulative Query and Index Statistics"
short_desc: "Collects timing statistics for WAL I/O activity."
context: "superuser"
default: "off"
url: "https://www.postgresql.org/docs/18/runtime-config-statistics.html#GUC-TRACK-WAL-IO-TIMING"
---

Enables timing of WAL I/O waits. This parameter is off by default, as it will repeatedly query the operating system for the current time, which may cause significant overhead on some platforms. You can use the pg_test_timing tool to measure the overhead of timing on your system. I/O timing information is displayed in [`pg_stat_io`](https://www.postgresql.org/docs/18/monitoring-stats.html#MONITORING-PG-STAT-IO-VIEW) for the `object` `wal` and in the output of the [`pg_stat_get_backend_io()`](https://www.postgresql.org/docs/18/monitoring-stats.html#PG-STAT-GET-BACKEND-IO) function for the `object` `wal`. Only superusers and users with the appropriate `SET` privilege can change this setting.
