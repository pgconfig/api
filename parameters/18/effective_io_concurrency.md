---
name: "effective_io_concurrency"
version: "18"
type: "integer"
category: "Resource Usage / I/O"
short_desc: "Number of simultaneous requests that can be handled efficiently by the disk subsystem."
extra_desc: "0 disables simultaneous requests."
context: "user"
default: "16"
min: "0"
max: "1000"
url: "https://www.postgresql.org/docs/18/runtime-config-resource.html#GUC-EFFECTIVE-IO-CONCURRENCY"
---

Sets the number of concurrent storage I/O operations that PostgreSQL expects can be executed simultaneously. Raising this value will increase the number of I/O operations that any individual PostgreSQL session attempts to initiate in parallel. The allowed range is `1` to `1000`, or `0` to disable issuance of asynchronous I/O requests. The default is `16`.

Higher values will have the most impact on higher latency storage where queries otherwise experience noticeable I/O stalls and on devices with high IOPs. Unnecessarily high values may increase I/O latency for all queries on the system.

On systems with prefetch advice support, `effective_io_concurrency` also controls the prefetch distance.

This value can be overridden for tables in a particular tablespace by setting the tablespace parameter of the same name (see [ALTER TABLESPACE](https://www.postgresql.org/docs/18/sql-altertablespace.html)).
