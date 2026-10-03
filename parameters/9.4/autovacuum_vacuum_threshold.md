---
name: "autovacuum_vacuum_threshold"
version: "9.4"
type: "integer"
category: "Autovacuum"
short_desc: "Minimum number of tuple updates or deletes prior to vacuum."
context: "sighup"
default: "50"
min: "0"
max: "2147483647"
url: "https://www.postgresql.org/docs/9.4/runtime-config-autovacuum.html#GUC-AUTOVACUUM-VACUUM-THRESHOLD"
---

Specifies the minimum number of updated or deleted tuples needed to trigger a `VACUUM` in any one table. The default is 50 tuples. This parameter can only be set in the `postgresql.conf` file or on the server command line. This setting can be overridden for individual tables by changing storage parameters.
