---
name: "archive_timeout"
version: "16"
type: "integer"
category: "Write-Ahead Log / Archiving"
short_desc: "Sets the amount of time to wait before forcing a switch to the next WAL file."
context: "sighup"
unit: "s"
default: "0"
min: "0"
max: "1073741823"
url: "https://www.postgresql.org/docs/16/runtime-config-wal.html#GUC-ARCHIVE-TIMEOUT"
---

The [`archive_command`](https://www.postgresql.org/docs/16/runtime-config-wal.html#GUC-ARCHIVE-COMMAND) or [`archive_library`](https://www.postgresql.org/docs/16/runtime-config-wal.html#GUC-ARCHIVE-LIBRARY) is only invoked for completed WAL segments. Hence, if your server generates little WAL traffic (or has slack periods where it does so), there could be a long delay between the completion of a transaction and its safe recording in archive storage. To limit how old unarchived data can be, you can set `archive_timeout` to force the server to switch to a new WAL segment file periodically. When this parameter is greater than zero, the server will switch to a new segment file whenever this amount of time has elapsed since the last segment file switch, and there has been any database activity, including a single checkpoint (checkpoints are skipped if there is no database activity). Note that archived files that are closed early due to a forced switch are still the same length as completely full files. Therefore, it is unwise to use a very short `archive_timeout` — it will bloat your archive storage. `archive_timeout` settings of a minute or so are usually reasonable. You should consider using streaming replication, instead of archiving, if you want data to be copied off the primary server more quickly than that. If this value is specified without units, it is taken as seconds. This parameter can only be set in the `postgresql.conf` file or on the server command line.
