---
name: "summarize_wal"
version: "19"
type: "boolean"
category: "Write-Ahead Log / Summarization"
short_desc: "Starts the WAL summarizer process to enable incremental backup."
context: "sighup"
default: "off"
url: "https://www.postgresql.org/docs/19/runtime-config-wal.html#GUC-SUMMARIZE-WAL"
---

Enables the WAL summarizer process. Note that WAL summarization can be enabled either on a primary or on a standby. This parameter can only be set in the `postgresql.conf` file or on the server command line. The default is `off`.

The server cannot be started with `summarize_wal=on` if `wal_level` is set to `minimal`. If `summarize_wal=on` is configured after server startup while `wal_level=minimal`, the summarizer will run but refuse to generate summary files for any WAL generated with `wal_level=minimal`.
