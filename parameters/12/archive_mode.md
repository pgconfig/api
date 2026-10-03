---
name: "archive_mode"
version: "12"
type: "enum"
category: "Write-Ahead Log / Archiving"
short_desc: "Allows archiving of WAL files using archive_command."
context: "postmaster"
default: "off"
values: ["always", "on", "off"]
url: "https://www.postgresql.org/docs/12/runtime-config-wal.html#GUC-ARCHIVE-MODE"
---

When `archive_mode` is enabled, completed WAL segments are sent to archive storage by setting [`archive_command`](https://www.postgresql.org/docs/12/runtime-config-wal.html#GUC-ARCHIVE-COMMAND). In addition to `off`, to disable, there are two modes: `on`, and `always`. During normal operation, there is no difference between the two modes, but when set to `always` the WAL archiver is enabled also during archive recovery or standby mode. In `always` mode, all files restored from the archive or streamed with streaming replication will be archived (again). See [Continuous Archiving in Standby](https://www.postgresql.org/docs/12/warm-standby.html#CONTINUOUS-ARCHIVING-IN-STANDBY) for details.

`archive_mode` and `archive_command` are separate variables so that `archive_command` can be changed without leaving archiving mode. This parameter can only be set at server start. `archive_mode` cannot be enabled when `wal_level` is set to `minimal`.
