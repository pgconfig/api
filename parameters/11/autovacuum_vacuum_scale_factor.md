---
name: "autovacuum_vacuum_scale_factor"
version: "11"
type: "floating point"
category: "Autovacuum"
short_desc: "Number of tuple updates or deletes prior to vacuum as a fraction of reltuples."
context: "sighup"
default: "0.2"
min: "0"
max: "100"
url: "https://www.postgresql.org/docs/11/runtime-config-autovacuum.html#GUC-AUTOVACUUM-VACUUM-SCALE-FACTOR"
---

Specifies a fraction of the table size to add to `autovacuum_vacuum_threshold` when deciding whether to trigger a `VACUUM`. The default is 0.2 (20% of table size). This parameter can only be set in the `postgresql.conf` file or on the server command line; but the setting can be overridden for individual tables by changing table storage parameters.
