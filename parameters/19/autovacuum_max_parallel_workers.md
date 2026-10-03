---
name: "autovacuum_max_parallel_workers"
version: "19"
type: "integer"
category: "Vacuuming / Automatic Vacuuming"
short_desc: "Maximum number of parallel workers that can be used by a single autovacuum worker."
context: "sighup"
default: "0"
min: "0"
max: "1024"
url: "https://www.postgresql.org/docs/19/runtime-config-vacuum.html#GUC-AUTOVACUUM-MAX-PARALLEL-WORKERS"
---

Sets the maximum number of parallel workers that can be used by a single autovacuum worker to process indexes. This limit applies specifically to the index vacuuming and index cleanup phases (for the details of each autovacuum phase, please refer to [VACUUM Phases](https://www.postgresql.org/docs/19/progress-reporting.html#VACUUM-PHASES)). The actual number of parallel workers is further limited by [`max_parallel_workers`](https://www.postgresql.org/docs/19/runtime-config-resource.html#GUC-MAX-PARALLEL-WORKERS). This is the per-autovacuum worker equivalent of the `PARALLEL` option of the [`VACUUM`](https://www.postgresql.org/docs/19/sql-vacuum.html) command. Setting this value to 0 disables parallel vacuum during autovacuum. The default is 0.
