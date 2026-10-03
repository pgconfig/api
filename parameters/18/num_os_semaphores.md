---
name: "num_os_semaphores"
version: "18"
type: "integer"
category: "Preset Options"
short_desc: "Shows the number of semaphores required for the server."
context: "internal"
default: "0"
min: "0"
max: "2147483647"
url: "https://www.postgresql.org/docs/18/runtime-config-preset.html#GUC-NUM-OS-SEMAPHORES"
---

Reports the number of semaphores that are needed for the server based on the configured number of allowed connections ([`max_connections`](https://www.postgresql.org/docs/18/runtime-config-connection.html#GUC-MAX-CONNECTIONS)), allowed autovacuum worker processes ([`autovacuum_max_workers`](https://www.postgresql.org/docs/18/runtime-config-vacuum.html#GUC-AUTOVACUUM-MAX-WORKERS)), allowed WAL sender processes ([`max_wal_senders`](https://www.postgresql.org/docs/18/runtime-config-replication.html#GUC-MAX-WAL-SENDERS)), allowed background processes ([`max_worker_processes`](https://www.postgresql.org/docs/18/runtime-config-resource.html#GUC-MAX-WORKER-PROCESSES)), etc.
