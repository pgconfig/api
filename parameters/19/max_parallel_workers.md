---
name: "max_parallel_workers"
version: "19"
type: "integer"
category: "Resource Usage / Worker Processes"
short_desc: "Sets the maximum number of parallel workers that can be active at one time."
context: "user"
default: "8"
min: "0"
max: "1024"
url: "https://www.postgresql.org/docs/19/runtime-config-resource.html#GUC-MAX-PARALLEL-WORKERS"
---

Sets the maximum number of workers that the cluster can support for parallel operations. The default value is 8. When increasing or decreasing this value, consider also adjusting [`max_parallel_maintenance_workers`](https://www.postgresql.org/docs/19/runtime-config-resource.html#GUC-MAX-PARALLEL-MAINTENANCE-WORKERS) and [`max_parallel_workers_per_gather`](https://www.postgresql.org/docs/19/runtime-config-resource.html#GUC-MAX-PARALLEL-WORKERS-PER-GATHER). Also, note that a setting for this value which is higher than [`max_worker_processes`](https://www.postgresql.org/docs/19/runtime-config-resource.html#GUC-MAX-WORKER-PROCESSES) will have no effect, since parallel workers are taken from the pool of worker processes established by that setting.
