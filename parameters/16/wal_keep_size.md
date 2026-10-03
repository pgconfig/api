---
name: "wal_keep_size"
version: "16"
type: "integer"
category: "Replication / Sending Servers"
short_desc: "Sets the size of WAL files held for standby servers."
context: "sighup"
unit: "MB"
default: "0"
min: "0"
max: "2147483647"
url: "https://www.postgresql.org/docs/16/runtime-config-replication.html#GUC-WAL-KEEP-SIZE"
---

Specifies the minimum size of past WAL files kept in the `pg_wal` directory, in case a standby server needs to fetch them for streaming replication. If a standby server connected to the sending server falls behind by more than `wal_keep_size` megabytes, the sending server might remove a WAL segment still needed by the standby, in which case the replication connection will be terminated. Downstream connections will also eventually fail as a result. (However, the standby server can recover by fetching the segment from archive, if WAL archiving is in use.)

This sets only the minimum size of segments retained in `pg_wal`; the system might need to retain more segments for WAL archival or to recover from a checkpoint. If `wal_keep_size` is zero (the default), the system doesn't keep any extra segments for standby purposes, so the number of old WAL segments available to standby servers is a function of the location of the previous checkpoint and status of WAL archiving. If this value is specified without units, it is taken as megabytes. This parameter can only be set in the `postgresql.conf` file or on the server command line.
