---
name: "recovery_prefetch"
version: "18"
type: "enum"
category: "Write-Ahead Log / Recovery"
short_desc: "Prefetch referenced blocks during recovery."
extra_desc: "Look ahead in the WAL to find references to uncached data."
context: "sighup"
default: "try"
values: ["off", "on", "try"]
url: "https://www.postgresql.org/docs/18/runtime-config-wal.html#GUC-RECOVERY-PREFETCH"
---

Whether to try to prefetch blocks that are referenced in the WAL that are not yet in the buffer pool, during recovery. Valid values are `off`, `on` and `try` (the default). The setting `try` enables prefetching only if the operating system provides support for issuing read-ahead advice.

Prefetching blocks that will soon be needed can reduce I/O wait times during recovery with some workloads. See also the [`wal_decode_buffer_size`](https://www.postgresql.org/docs/18/runtime-config-wal.html#GUC-WAL-DECODE-BUFFER-SIZE) and [`maintenance_io_concurrency`](https://www.postgresql.org/docs/18/runtime-config-resource.html#GUC-MAINTENANCE-IO-CONCURRENCY) settings, which limit prefetching activity.
