---
name: "wal_writer_delay"
version: "9.5"
type: "integer"
category: "Write-Ahead Log / Settings"
short_desc: "WAL writer sleep time between WAL flushes."
context: "sighup"
unit: "ms"
default: "200"
min: "1"
max: "10000"
url: "https://www.postgresql.org/docs/9.5/runtime-config-wal.html#GUC-WAL-WRITER-DELAY"
---

Specifies the delay between activity rounds for the WAL writer. In each round the writer will flush WAL to disk. It then sleeps for `wal_writer_delay` milliseconds, and repeats. The default value is 200 milliseconds (`200ms`). Note that on many systems, the effective resolution of sleep delays is 10 milliseconds; setting `wal_writer_delay` to a value that is not a multiple of 10 might have the same results as setting it to the next higher multiple of 10. This parameter can only be set in the `postgresql.conf` file or on the server command line.
