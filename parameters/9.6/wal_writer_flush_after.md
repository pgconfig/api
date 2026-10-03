---
name: "wal_writer_flush_after"
version: "9.6"
type: "integer"
category: "Write-Ahead Log / Settings"
short_desc: "Amount of WAL written out by WAL writer that triggers a flush."
context: "sighup"
unit: "8kB"
default: "128"
min: "0"
max: "2147483647"
url: "https://www.postgresql.org/docs/9.6/runtime-config-wal.html#GUC-WAL-WRITER-FLUSH-AFTER"
---

Specifies how often the WAL writer flushes WAL. If the last flush happened less than `wal_writer_delay` milliseconds ago and less than `wal_writer_flush_after` bytes of WAL have been produced since, then WAL is only written to the operating system, not flushed to disk. If `wal_writer_flush_after` is set to `0` then WAL data is flushed immediately. The default is `1MB`. This parameter can only be set in the `postgresql.conf` file or on the server command line.
