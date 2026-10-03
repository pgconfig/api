---
name: "autovacuum_work_mem"
version: "19"
type: "integer"
category: "Resource Usage / Memory"
short_desc: "Sets the maximum memory to be used by each autovacuum worker process."
extra_desc: "-1 means use \"maintenance_work_mem\"."
context: "sighup"
unit: "kB"
default: "-1"
min: "-1"
max: "2147483647"
url: "https://www.postgresql.org/docs/19/runtime-config-resource.html#GUC-AUTOVACUUM-WORK-MEM"
---

Specifies the maximum amount of memory to be used by each autovacuum worker process. If this value is specified without units, it is taken as kilobytes. It defaults to -1, indicating that the value of [`maintenance_work_mem`](https://www.postgresql.org/docs/19/runtime-config-resource.html#GUC-MAINTENANCE-WORK-MEM) should be used instead. The setting has no effect on the behavior of `VACUUM` when run in other contexts. This parameter can only be set in the `postgresql.conf` file or on the server command line.
