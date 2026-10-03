---
name: "max_logical_replication_workers"
version: "10"
type: "integer"
category: "Replication / Subscribers"
short_desc: "Maximum number of logical replication worker processes."
context: "postmaster"
default: "4"
min: "0"
max: "262143"
url: "https://www.postgresql.org/docs/10/runtime-config-replication.html#GUC-MAX-LOGICAL-REPLICATION-WORKERS"
---

Specifies maximum number of logical replication workers. This includes both apply workers and table synchronization workers.

Logical replication workers are taken from the pool defined by `max_worker_processes`.

The default value is 4. This parameter can only be set at server start.
