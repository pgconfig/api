---
name: "wal_writer_flush_after"
version: "15"
type: "integer"
category: "Write-Ahead Log / Settings"
short_desc: "Amount of WAL written out by WAL writer that triggers a flush."
context: "sighup"
unit: "8kB"
default: "128"
min: "0"
max: "2147483647"
url: "https://www.postgresql.org/docs/15/runtime-config-wal.html#GUC-WAL-WRITER-FLUSH-AFTER"
---

Specifies how often the WAL writer flushes WAL, in volume terms. If the last flush happened less than `wal_writer_delay` ago and less than `wal_writer_flush_after` worth of WAL has been produced since, then WAL is only written to the operating system, not flushed to disk. If `wal_writer_flush_after` is set to `0` then WAL data is always flushed immediately. If this value is specified without units, it is taken as WAL blocks, that is `XLOG_BLCKSZ` bytes, typically 8kB. The default is `1MB`. This parameter can only be set in the `postgresql.conf` file or on the server command line.
