---
name: "wal_writer_delay"
version: "10"
type: "integer"
category: "Write-Ahead Log / Settings"
short_desc: "Time between WAL flushes performed in the WAL writer."
context: "sighup"
unit: "ms"
default: "200"
min: "1"
max: "10000"
url: "https://www.postgresql.org/docs/10/runtime-config-wal.html#GUC-WAL-WRITER-DELAY"
---

Specifies how often the WAL writer flushes WAL. After flushing WAL it sleeps for `wal_writer_delay` milliseconds, unless woken up by an asynchronously committing transaction. If the last flush happened less than `wal_writer_delay` milliseconds ago and less than `wal_writer_flush_after` bytes of WAL have been produced since, then WAL is only written to the operating system, not flushed to disk. The default value is 200 milliseconds (`200ms`). Note that on many systems, the effective resolution of sleep delays is 10 milliseconds; setting `wal_writer_delay` to a value that is not a multiple of 10 might have the same results as setting it to the next higher multiple of 10. This parameter can only be set in the `postgresql.conf` file or on the server command line.
