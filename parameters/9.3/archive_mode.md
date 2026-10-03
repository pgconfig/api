---
name: "archive_mode"
version: "9.3"
type: "boolean"
category: "Write-Ahead Log / Archiving"
short_desc: "Allows archiving of WAL files using archive_command."
context: "postmaster"
default: "off"
url: "https://www.postgresql.org/docs/9.3/runtime-config-wal.html#GUC-ARCHIVE-MODE"
---

When `archive_mode` is enabled, completed WAL segments are sent to archive storage by setting [`archive_command`](https://www.postgresql.org/docs/9.3/runtime-config-wal.html#GUC-ARCHIVE-COMMAND). `archive_mode` and `archive_command` are separate variables so that `archive_command` can be changed without leaving archiving mode. This parameter can only be set at server start. `archive_mode` cannot be enabled when `wal_level` is set to `minimal`.
