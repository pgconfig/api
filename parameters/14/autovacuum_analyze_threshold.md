---
name: "autovacuum_analyze_threshold"
version: "14"
type: "integer"
category: "Autovacuum"
short_desc: "Minimum number of tuple inserts, updates, or deletes prior to analyze."
context: "sighup"
default: "50"
min: "0"
max: "2147483647"
url: "https://www.postgresql.org/docs/14/runtime-config-autovacuum.html#GUC-AUTOVACUUM-ANALYZE-THRESHOLD"
---

Specifies the minimum number of inserted, updated or deleted tuples needed to trigger an `ANALYZE` in any one table. The default is 50 tuples. This parameter can only be set in the `postgresql.conf` file or on the server command line; but the setting can be overridden for individual tables by changing table storage parameters.
