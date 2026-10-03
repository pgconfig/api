---
name: "wal_keep_segments"
version: "10"
type: "integer"
category: "Replication / Sending Servers"
short_desc: "Sets the number of WAL files held for standby servers."
context: "sighup"
default: "0"
min: "0"
max: "2147483647"
url: "https://www.postgresql.org/docs/10/runtime-config-replication.html#GUC-WAL-KEEP-SEGMENTS"
---

Specifies the minimum number of past log file segments kept in the `pg_wal` directory, in case a standby server needs to fetch them for streaming replication. Each segment is normally 16 megabytes. If a standby server connected to the sending server falls behind by more than `wal_keep_segments` segments, the sending server might remove a WAL segment still needed by the standby, in which case the replication connection will be terminated. Downstream connections will also eventually fail as a result. (However, the standby server can recover by fetching the segment from archive, if WAL archiving is in use.)

This sets only the minimum number of segments retained in `pg_wal`; the system might need to retain more segments for WAL archival or to recover from a checkpoint. If `wal_keep_segments` is zero (the default), the system doesn't keep any extra segments for standby purposes, so the number of old WAL segments available to standby servers is a function of the location of the previous checkpoint and status of WAL archiving. This parameter can only be set in the `postgresql.conf` file or on the server command line.
