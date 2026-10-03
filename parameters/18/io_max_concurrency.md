---
name: "io_max_concurrency"
version: "18"
type: "integer"
category: "Resource Usage / I/O"
short_desc: "Max number of IOs that one process can execute simultaneously."
context: "postmaster"
default: "-1"
min: "-1"
max: "1024"
url: "https://www.postgresql.org/docs/18/runtime-config-resource.html#GUC-IO-MAX-CONCURRENCY"
---

Controls the maximum number of I/O operations that one process can execute simultaneously.

The default setting of `-1` selects a number based on [`shared_buffers`](https://www.postgresql.org/docs/18/runtime-config-resource.html#GUC-SHARED-BUFFERS) and the maximum number of processes ([`max_connections`](https://www.postgresql.org/docs/18/runtime-config-connection.html#GUC-MAX-CONNECTIONS), [`autovacuum_worker_slots`](https://www.postgresql.org/docs/18/runtime-config-vacuum.html#GUC-AUTOVACUUM-WORKER-SLOTS), [`max_worker_processes`](https://www.postgresql.org/docs/18/runtime-config-resource.html#GUC-MAX-WORKER-PROCESSES) and [`max_wal_senders`](https://www.postgresql.org/docs/18/runtime-config-replication.html#GUC-MAX-WAL-SENDERS)), but not more than `64`.

This parameter can only be set at server start.
