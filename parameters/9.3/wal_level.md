---
name: "wal_level"
version: "9.3"
type: "enum"
category: "Write-Ahead Log / Settings"
short_desc: "Set the level of information written to the WAL."
context: "postmaster"
default: "minimal"
values: ["minimal", "archive", "hot_standby"]
url: "https://www.postgresql.org/docs/9.3/runtime-config-wal.html#GUC-WAL-LEVEL"
---

`wal_level` determines how much information is written to the WAL. The default value is `minimal`, which writes only the information needed to recover from a crash or immediate shutdown. `archive` adds logging required for WAL archiving, and `hot_standby` further adds information required to run read-only queries on a standby server. This parameter can only be set at server start.

In `minimal` level, WAL-logging of some bulk operations can be safely skipped, which can make those operations much faster (see [Disable WAL Archival and Streaming Replication](https://www.postgresql.org/docs/9.3/populate.html#POPULATE-PITR)). Operations in which this optimization can be applied include:

- `CREATE TABLE AS`
- `CREATE INDEX`
- `CLUSTER`
- `COPY` into tables that were created or truncated in the same transaction

But minimal WAL does not contain enough information to reconstruct the data from a base backup and the WAL logs, so either `archive` or `hot_standby` level must be used to enable WAL archiving ([`archive_mode`](https://www.postgresql.org/docs/9.3/runtime-config-wal.html#GUC-ARCHIVE-MODE)) and streaming replication.

In `hot_standby` level, the same information is logged as with `archive`, plus information needed to reconstruct the status of running transactions from the WAL. To enable read-only queries on a standby server, `wal_level` must be set to `hot_standby` on the primary, and [`hot_standby`](https://www.postgresql.org/docs/9.3/runtime-config-replication.html#GUC-HOT-STANDBY) must be enabled in the standby. It is thought that there is little measurable difference in performance between using `hot_standby` and `archive` levels, so feedback is welcome if any production impacts are noticeable.
