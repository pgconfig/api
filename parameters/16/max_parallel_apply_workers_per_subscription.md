---
name: "max_parallel_apply_workers_per_subscription"
version: "16"
type: "integer"
category: "Replication / Subscribers"
short_desc: "Maximum number of parallel apply workers per subscription."
context: "sighup"
default: "2"
min: "0"
max: "1024"
url: "https://www.postgresql.org/docs/16/runtime-config-replication.html#GUC-MAX-PARALLEL-APPLY-WORKERS-PER-SUBSCRIPTION"
---

Maximum number of parallel apply workers per subscription. This parameter controls the amount of parallelism for streaming of in-progress transactions with subscription parameter `streaming = parallel`.

The parallel apply workers are taken from the pool defined by `max_logical_replication_workers`.

The default value is 2. This parameter can only be set in the `postgresql.conf` file or on the server command line.
