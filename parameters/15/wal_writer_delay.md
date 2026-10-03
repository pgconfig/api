---
name: "wal_writer_delay"
version: "15"
type: "integer"
category: "Write-Ahead Log / Settings"
short_desc: "Time between WAL flushes performed in the WAL writer."
context: "sighup"
unit: "ms"
default: "200"
min: "1"
max: "10000"
url: "https://www.postgresql.org/docs/15/runtime-config-wal.html#GUC-WAL-WRITER-DELAY"
---

Specifies how often the WAL writer flushes WAL, in time terms. After flushing WAL the writer sleeps for the length of time given by `wal_writer_delay`, unless woken up sooner by an asynchronously committing transaction. If the last flush happened less than `wal_writer_delay` ago and less than `wal_writer_flush_after` worth of WAL has been produced since, then WAL is only written to the operating system, not flushed to disk. If this value is specified without units, it is taken as milliseconds. The default value is 200 milliseconds (`200ms`). Note that on many systems, the effective resolution of sleep delays is 10 milliseconds; setting `wal_writer_delay` to a value that is not a multiple of 10 might have the same results as setting it to the next higher multiple of 10. This parameter can only be set in the `postgresql.conf` file or on the server command line.
