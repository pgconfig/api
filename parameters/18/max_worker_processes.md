---
name: "max_worker_processes"
version: "18"
type: "integer"
category: "Resource Usage / Worker Processes"
short_desc: "Maximum number of concurrent worker processes."
context: "postmaster"
default: "8"
min: "0"
max: "262143"
url: "https://www.postgresql.org/docs/18/runtime-config-resource.html#GUC-MAX-WORKER-PROCESSES"
---

Sets the maximum number of background processes that the cluster can support. This parameter can only be set at server start. The default is 8.

When running a standby server, you must set this parameter to the same or higher value than on the primary server. Otherwise, queries will not be allowed in the standby server.

When changing this value, consider also adjusting [`max_parallel_workers`](https://www.postgresql.org/docs/18/runtime-config-resource.html#GUC-MAX-PARALLEL-WORKERS), [`max_parallel_maintenance_workers`](https://www.postgresql.org/docs/18/runtime-config-resource.html#GUC-MAX-PARALLEL-MAINTENANCE-WORKERS), and [`max_parallel_workers_per_gather`](https://www.postgresql.org/docs/18/runtime-config-resource.html#GUC-MAX-PARALLEL-WORKERS-PER-GATHER).
