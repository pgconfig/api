---
name: "autovacuum_analyze_threshold"
version: "18"
type: "integer"
category: "Vacuuming / Automatic Vacuuming"
short_desc: "Minimum number of tuple inserts, updates, or deletes prior to analyze."
context: "sighup"
default: "50"
min: "0"
max: "2147483647"
url: "https://www.postgresql.org/docs/18/runtime-config-vacuum.html#GUC-AUTOVACUUM-ANALYZE-THRESHOLD"
---

Specifies the minimum number of inserted, updated or deleted tuples needed to trigger an `ANALYZE` in any one table. The default is 50 tuples. This parameter can only be set in the `postgresql.conf` file or on the server command line; but the setting can be overridden for individual tables by changing table storage parameters.
