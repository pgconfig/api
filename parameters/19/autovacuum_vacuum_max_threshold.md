---
name: "autovacuum_vacuum_max_threshold"
version: "19"
type: "integer"
category: "Vacuuming / Automatic Vacuuming"
short_desc: "Maximum number of tuple updates or deletes prior to vacuum."
extra_desc: "-1 disables the maximum threshold."
context: "sighup"
default: "100000000"
min: "-1"
max: "2147483647"
url: "https://www.postgresql.org/docs/19/runtime-config-vacuum.html#GUC-AUTOVACUUM-VACUUM-MAX-THRESHOLD"
---

Specifies the maximum number of updated or deleted tuples needed to trigger a `VACUUM` in any one table, i.e., a limit on the value calculated with `autovacuum_vacuum_threshold` and `autovacuum_vacuum_scale_factor`. The default is 100,000,000 tuples. If -1 is specified, autovacuum will not enforce a maximum number of updated or deleted tuples that will trigger a `VACUUM` operation. This parameter can only be set in the `postgresql.conf` file or on the server command line; but the setting can be overridden for individual tables by changing storage parameters.
