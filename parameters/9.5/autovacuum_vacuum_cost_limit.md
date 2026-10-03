---
name: "autovacuum_vacuum_cost_limit"
version: "9.5"
type: "integer"
category: "Autovacuum"
short_desc: "Vacuum cost amount available before napping, for autovacuum."
context: "sighup"
default: "-1"
min: "-1"
max: "10000"
url: "https://www.postgresql.org/docs/9.5/runtime-config-autovacuum.html#GUC-AUTOVACUUM-VACUUM-COST-LIMIT"
---

Specifies the cost limit value that will be used in automatic `VACUUM` operations. If -1 is specified (which is the default), the regular [`vacuum_cost_limit`](https://www.postgresql.org/docs/9.5/runtime-config-resource.html#GUC-VACUUM-COST-LIMIT) value will be used. Note that the value is distributed proportionally among the running autovacuum workers, if there is more than one, so that the sum of the limits for each worker does not exceed the value of this variable. This parameter can only be set in the `postgresql.conf` file or on the server command line; but the setting can be overridden for individual tables by changing table storage parameters.
