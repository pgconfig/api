---
name: "autovacuum_vacuum_cost_delay"
version: "13"
type: "floating point"
category: "Autovacuum"
short_desc: "Vacuum cost delay in milliseconds, for autovacuum."
context: "sighup"
unit: "ms"
default: "2"
min: "-1"
max: "100"
url: "https://www.postgresql.org/docs/13/runtime-config-autovacuum.html#GUC-AUTOVACUUM-VACUUM-COST-DELAY"
---

Specifies the cost delay value that will be used in automatic `VACUUM` operations. If -1 is specified, the regular [`vacuum_cost_delay`](https://www.postgresql.org/docs/13/runtime-config-resource.html#GUC-VACUUM-COST-DELAY) value will be used. If this value is specified without units, it is taken as milliseconds. The default value is 2 milliseconds. This parameter can only be set in the `postgresql.conf` file or on the server command line; but the setting can be overridden for individual tables by changing table storage parameters.
