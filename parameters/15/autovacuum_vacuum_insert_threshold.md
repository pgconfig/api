---
name: "autovacuum_vacuum_insert_threshold"
version: "15"
type: "integer"
category: "Autovacuum"
short_desc: "Minimum number of tuple inserts prior to vacuum, or -1 to disable insert vacuums."
context: "sighup"
default: "1000"
min: "-1"
max: "2147483647"
url: "https://www.postgresql.org/docs/15/runtime-config-autovacuum.html#GUC-AUTOVACUUM-VACUUM-INSERT-THRESHOLD"
---

Specifies the number of inserted tuples needed to trigger a `VACUUM` in any one table. The default is 1000 tuples. If -1 is specified, autovacuum will not trigger a `VACUUM` operation on any tables based on the number of inserts. This parameter can only be set in the `postgresql.conf` file or on the server command line; but the setting can be overridden for individual tables by changing table storage parameters.
