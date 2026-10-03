---
name: "wal_compression"
version: "19"
type: "enum"
category: "Write-Ahead Log / Settings"
short_desc: "Compresses full-page writes written in WAL file with specified method."
context: "superuser"
default: "off"
values: ["pglz", "lz4", "zstd", "on", "off"]
url: "https://www.postgresql.org/docs/19/runtime-config-wal.html#GUC-WAL-COMPRESSION"
---

This parameter enables compression of WAL using the specified compression method. When enabled, the PostgreSQL server compresses full page images written to WAL (e.g. when [`full_page_writes`](https://www.postgresql.org/docs/19/runtime-config-wal.html#GUC-FULL-PAGE-WRITES) is on, during a base backup, etc.). A compressed page image will be decompressed during WAL replay. The supported methods are `off`, `on`, `zstd` (if PostgreSQL was compiled with `--with-zstd`), `lz4` (if PostgreSQL was compiled with `--with-lz4`), and `pglz`. The value `on` selects the first of `zstd`, `lz4`, `pglz` that is available. The default value is `off`. Only superusers and users with the appropriate `SET` privilege can change this setting.

Enabling compression can reduce the WAL volume without increasing the risk of unrecoverable data corruption, but at the cost of some extra CPU spent on the compression during WAL logging and on the decompression during WAL replay.
