---
name: "wal_keep_segments"
version: "9.1"
type: "integer"
category: "Replication / Master Server"
short_desc: "Sets the number of WAL files held for standby servers."
context: "sighup"
default: "0"
min: "0"
max: "2147483647"
url: "https://www.postgresql.org/docs/9.1/runtime-config-replication.html#GUC-WAL-KEEP-SEGMENTS"
---

Specifies the minimum number of past log file segments kept in the `pg_xlog` directory, in case a standby server needs to fetch them for streaming replication. Each segment is normally 16 megabytes. If a standby server connected to the primary falls behind by more than `wal_keep_segments` segments, the primary might remove a WAL segment still needed by the standby, in which case the replication connection will be terminated. (However, the standby server can recover by fetching the segment from archive, if WAL archiving is in use.)

This sets only the minimum number of segments retained in `pg_xlog`; the system might need to retain more segments for WAL archival or to recover from a checkpoint. If `wal_keep_segments` is zero (the default), the system doesn't keep any extra segments for standby purposes, so the number of old WAL segments available to standby servers is a function of the location of the previous checkpoint and status of WAL archiving. This parameter has no effect on restartpoints. This parameter can only be set in the `postgresql.conf` file or on the server command line.
