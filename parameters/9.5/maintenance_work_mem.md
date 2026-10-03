---
name: "maintenance_work_mem"
version: "9.5"
type: "integer"
category: "Resource Usage / Memory"
short_desc: "Sets the maximum memory to be used for maintenance operations."
extra_desc: "This includes operations such as VACUUM and CREATE INDEX."
context: "user"
unit: "kB"
default: "65536"
min: "1024"
max: "2147483647"
url: "https://www.postgresql.org/docs/9.5/runtime-config-resource.html#GUC-MAINTENANCE-WORK-MEM"
---

Specifies the maximum amount of memory to be used by maintenance operations, such as `VACUUM`, `CREATE INDEX`, and `ALTER TABLE ADD FOREIGN KEY`. It defaults to 64 megabytes (`64MB`). Since only one of these operations can be executed at a time by a database session, and an installation normally doesn't have many of them running concurrently, it's safe to set this value significantly larger than `work_mem`. Larger settings might improve performance for vacuuming and for restoring database dumps.

Note that when autovacuum runs, up to [`autovacuum_max_workers`](https://www.postgresql.org/docs/9.5/runtime-config-autovacuum.html#GUC-AUTOVACUUM-MAX-WORKERS) times this memory may be allocated, so be careful not to set the default value too high. It may be useful to control for this by separately setting [`autovacuum_work_mem`](https://www.postgresql.org/docs/9.5/runtime-config-resource.html#GUC-AUTOVACUUM-WORK-MEM).
