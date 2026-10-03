---
name: "autovacuum_work_mem"
version: "9.4"
type: "integer"
category: "Resource Usage / Memory"
short_desc: "Sets the maximum memory to be used by each autovacuum worker process."
context: "sighup"
unit: "kB"
default: "-1"
min: "-1"
max: "2147483647"
url: "https://www.postgresql.org/docs/9.4/runtime-config-resource.html#GUC-AUTOVACUUM-WORK-MEM"
---

Specifies the maximum amount of memory to be used by each autovacuum worker process. It defaults to -1, indicating that the value of [`maintenance_work_mem`](https://www.postgresql.org/docs/9.4/runtime-config-resource.html#GUC-MAINTENANCE-WORK-MEM) should be used instead. The setting has no effect on the behavior of `VACUUM` when run in other contexts.
