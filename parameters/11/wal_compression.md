---
name: "wal_compression"
version: "11"
type: "boolean"
category: "Write-Ahead Log / Settings"
short_desc: "Compresses full-page writes written in WAL file."
context: "superuser"
default: "off"
url: "https://www.postgresql.org/docs/11/runtime-config-wal.html#GUC-WAL-COMPRESSION"
---

When this parameter is `on`, the PostgreSQL server compresses a full page image written to WAL when [`full_page_writes`](https://www.postgresql.org/docs/11/runtime-config-wal.html#GUC-FULL-PAGE-WRITES) is on or during a base backup. A compressed page image will be decompressed during WAL replay. The default value is `off`. Only superusers can change this setting.

Turning this parameter on can reduce the WAL volume without increasing the risk of unrecoverable data corruption, but at the cost of some extra CPU spent on the compression during WAL logging and on the decompression during WAL replay.
