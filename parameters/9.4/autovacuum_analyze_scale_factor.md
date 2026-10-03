---
name: "autovacuum_analyze_scale_factor"
version: "9.4"
type: "floating point"
category: "Autovacuum"
short_desc: "Number of tuple inserts, updates, or deletes prior to analyze as a fraction of reltuples."
context: "sighup"
default: "0.1"
min: "0"
max: "100"
url: "https://www.postgresql.org/docs/9.4/runtime-config-autovacuum.html#GUC-AUTOVACUUM-ANALYZE-SCALE-FACTOR"
---

Specifies a fraction of the table size to add to `autovacuum_analyze_threshold` when deciding whether to trigger an `ANALYZE`. The default is 0.1 (10% of table size). This parameter can only be set in the `postgresql.conf` file or on the server command line. This setting can be overridden for individual tables by changing storage parameters.
