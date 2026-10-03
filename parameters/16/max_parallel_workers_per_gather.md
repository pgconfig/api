---
name: "max_parallel_workers_per_gather"
version: "16"
type: "integer"
category: "Resource Usage / Asynchronous Behavior"
short_desc: "Sets the maximum number of parallel processes per executor node."
context: "user"
default: "2"
min: "0"
max: "1024"
url: "https://www.postgresql.org/docs/16/runtime-config-resource.html#GUC-MAX-PARALLEL-WORKERS-PER-GATHER"
---

Sets the maximum number of workers that can be started by a single `Gather` or `Gather Merge` node. Parallel workers are taken from the pool of processes established by [`max_worker_processes`](https://www.postgresql.org/docs/16/runtime-config-resource.html#GUC-MAX-WORKER-PROCESSES), limited by [`max_parallel_workers`](https://www.postgresql.org/docs/16/runtime-config-resource.html#GUC-MAX-PARALLEL-WORKERS). Note that the requested number of workers may not actually be available at run time. If this occurs, the plan will run with fewer workers than expected, which may be inefficient. The default value is 2. Setting this value to 0 disables parallel query execution.

Note that parallel queries may consume very substantially more resources than non-parallel queries, because each worker process is a completely separate process which has roughly the same impact on the system as an additional user session. This should be taken into account when choosing a value for this setting, as well as when configuring other settings that control resource utilization, such as [`work_mem`](https://www.postgresql.org/docs/16/runtime-config-resource.html#GUC-WORK-MEM). Resource limits such as `work_mem` are applied individually to each worker, which means the total utilization may be much higher across all processes than it would normally be for any single process. For example, a parallel query using 4 workers may use up to 5 times as much CPU time, memory, I/O bandwidth, and so forth as a query which uses no workers at all.

For more information on parallel query, see [Parallel Query](https://www.postgresql.org/docs/16/parallel-query.html).
