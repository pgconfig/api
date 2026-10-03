---
name: "max_parallel_maintenance_workers"
version: "16"
type: "integer"
category: "Resource Usage / Asynchronous Behavior"
short_desc: "Sets the maximum number of parallel processes per maintenance operation."
context: "user"
default: "2"
min: "0"
max: "1024"
url: "https://www.postgresql.org/docs/16/runtime-config-resource.html#GUC-MAX-PARALLEL-MAINTENANCE-WORKERS"
---

Sets the maximum number of parallel workers that can be started by a single utility command. Currently, the parallel utility commands that support the use of parallel workers are `CREATE INDEX` only when building a B-tree index, and `VACUUM` without `FULL` option. Parallel workers are taken from the pool of processes established by [`max_worker_processes`](https://www.postgresql.org/docs/16/runtime-config-resource.html#GUC-MAX-WORKER-PROCESSES), limited by [`max_parallel_workers`](https://www.postgresql.org/docs/16/runtime-config-resource.html#GUC-MAX-PARALLEL-WORKERS). Note that the requested number of workers may not actually be available at run time. If this occurs, the utility operation will run with fewer workers than expected. The default value is 2. Setting this value to 0 disables the use of parallel workers by utility commands.

Note that parallel utility commands should not consume substantially more memory than equivalent non-parallel operations. This strategy differs from that of parallel query, where resource limits generally apply per worker process. Parallel utility commands treat the resource limit `maintenance_work_mem` as a limit to be applied to the entire utility command, regardless of the number of parallel worker processes. However, parallel utility commands may still consume substantially more CPU resources and I/O bandwidth.
