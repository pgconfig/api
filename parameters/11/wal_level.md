---
name: "wal_level"
version: "11"
type: "enum"
category: "Write-Ahead Log / Settings"
short_desc: "Set the level of information written to the WAL."
context: "postmaster"
default: "replica"
values: ["minimal", "replica", "logical"]
url: "https://www.postgresql.org/docs/11/runtime-config-wal.html#GUC-WAL-LEVEL"
---

`wal_level` determines how much information is written to the WAL. The default value is `replica`, which writes enough data to support WAL archiving and replication, including running read-only queries on a standby server. `minimal` removes all logging except the information required to recover from a crash or immediate shutdown. Finally, `logical` adds information necessary to support logical decoding. Each level includes the information logged at all lower levels. This parameter can only be set at server start.

In `minimal` level, WAL-logging of some bulk operations can be safely skipped, which can make those operations much faster (see [Disable WAL Archival and Streaming Replication](https://www.postgresql.org/docs/11/populate.html#POPULATE-PITR)). Operations in which this optimization can be applied include:

- `CREATE TABLE AS`
- `CREATE INDEX`
- `CLUSTER`
- `COPY` into tables that were created or truncated in the same transaction

But minimal WAL does not contain enough information to reconstruct the data from a base backup and the WAL logs, so `replica` or higher must be used to enable WAL archiving ([`archive_mode`](https://www.postgresql.org/docs/11/runtime-config-wal.html#GUC-ARCHIVE-MODE)) and streaming replication.

In `logical` level, the same information is logged as with `replica`, plus information needed to allow extracting logical change sets from the WAL. Using a level of `logical` will increase the WAL volume, particularly if many tables are configured for `REPLICA IDENTITY FULL` and many `UPDATE` and `DELETE` statements are executed.

In releases prior to 9.6, this parameter also allowed the values `archive` and `hot_standby`. These are still accepted but mapped to `replica`.
