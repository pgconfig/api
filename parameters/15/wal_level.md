---
name: "wal_level"
version: "15"
type: "enum"
category: "Write-Ahead Log / Settings"
short_desc: "Sets the level of information written to the WAL."
context: "postmaster"
default: "replica"
values: ["minimal", "replica", "logical"]
url: "https://www.postgresql.org/docs/15/runtime-config-wal.html#GUC-WAL-LEVEL"
---

`wal_level` determines how much information is written to the WAL. The default value is `replica`, which writes enough data to support WAL archiving and replication, including running read-only queries on a standby server. `minimal` removes all logging except the information required to recover from a crash or immediate shutdown. Finally, `logical` adds information necessary to support logical decoding. Each level includes the information logged at all lower levels. This parameter can only be set at server start.

The `minimal` level generates the least WAL volume. It logs no row information for permanent relations in transactions that create or rewrite them. This can make operations much faster (see [Disable WAL Archival and Streaming Replication](https://www.postgresql.org/docs/15/populate.html#POPULATE-PITR)). Operations that initiate this optimization include:

- `ALTER ... SET TABLESPACE`
- `CLUSTER`
- `CREATE TABLE`
- `REFRESH MATERIALIZED VIEW` (without `CONCURRENTLY`)
- `REINDEX`
- `TRUNCATE`

However, minimal WAL does not contain sufficient information for point-in-time recovery, so `replica` or higher must be used to enable continuous archiving ([`archive_mode`](https://www.postgresql.org/docs/15/runtime-config-wal.html#GUC-ARCHIVE-MODE)) and streaming binary replication. In fact, the server will not even start in this mode if `max_wal_senders` is non-zero. Note that changing `wal_level` to `minimal` makes previous base backups unusable for point-in-time recovery and standby servers.

In `logical` level, the same information is logged as with `replica`, plus information needed to extract logical change sets from the WAL. Using a level of `logical` will increase the WAL volume, particularly if many tables are configured for `REPLICA IDENTITY FULL` and many `UPDATE` and `DELETE` statements are executed.

In releases prior to 9.6, this parameter also allowed the values `archive` and `hot_standby`. These are still accepted but mapped to `replica`.
