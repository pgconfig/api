---
name: "archive_timeout"
version: "9.3"
type: "integer"
category: "Write-Ahead Log / Archiving"
short_desc: "Forces a switch to the next xlog file if a new file has not been started within N seconds."
context: "sighup"
unit: "s"
default: "0"
min: "0"
max: "1073741823"
url: "https://www.postgresql.org/docs/9.3/runtime-config-wal.html#GUC-ARCHIVE-TIMEOUT"
---

The [`archive_command`](https://www.postgresql.org/docs/9.3/runtime-config-wal.html#GUC-ARCHIVE-COMMAND) is only invoked for completed WAL segments. Hence, if your server generates little WAL traffic (or has slack periods where it does so), there could be a long delay between the completion of a transaction and its safe recording in archive storage. To limit how old unarchived data can be, you can set `archive_timeout` to force the server to switch to a new WAL segment file periodically. When this parameter is greater than zero, the server will switch to a new segment file whenever this many seconds have elapsed since the last segment file switch, and there has been any database activity, including a single checkpoint. (Increasing `checkpoint_timeout` will reduce unnecessary checkpoints on an idle system.) Note that archived files that are closed early due to a forced switch are still the same length as completely full files. Therefore, it is unwise to use a very short `archive_timeout` — it will bloat your archive storage. `archive_timeout` settings of a minute or so are usually reasonable. You should consider using streaming replication, instead of archiving, if you want data to be copied off the master server more quickly than that. This parameter can only be set in the `postgresql.conf` file or on the server command line.
