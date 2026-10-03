---
name: "synchronize_seqscans"
version: "12"
type: "boolean"
category: "Version and Platform Compatibility / Previous PostgreSQL Versions"
short_desc: "Enable synchronized sequential scans."
context: "user"
default: "on"
url: "https://www.postgresql.org/docs/12/runtime-config-compatible.html#GUC-SYNCHRONIZE-SEQSCANS"
---

This allows sequential scans of large tables to synchronize with each other, so that concurrent scans read the same block at about the same time and hence share the I/O workload. When this is enabled, a scan might start in the middle of the table and then "wrap around" the end to cover all rows, so as to synchronize with the activity of scans already in progress. This can result in unpredictable changes in the row ordering returned by queries that have no `ORDER BY` clause. Setting this parameter to `off` ensures the pre-8.3 behavior in which a sequential scan always starts from the beginning of the table. The default is `on`.
