---
name: "max_standby_archive_delay"
version: "10"
type: "integer"
category: "Replication / Standby Servers"
short_desc: "Sets the maximum delay before canceling queries when a hot standby server is processing archived WAL data."
context: "sighup"
unit: "ms"
default: "30000"
min: "-1"
max: "2147483647"
url: "https://www.postgresql.org/docs/10/runtime-config-replication.html#GUC-MAX-STANDBY-ARCHIVE-DELAY"
---

When Hot Standby is active, this parameter determines how long the standby server should wait before canceling standby queries that conflict with about-to-be-applied WAL entries, as described in [Handling Query Conflicts](https://www.postgresql.org/docs/10/hot-standby.html#HOT-STANDBY-CONFLICT). `max_standby_archive_delay` applies when WAL data is being read from WAL archive (and is therefore not current). The default is 30 seconds. Units are milliseconds if not specified. A value of -1 allows the standby to wait forever for conflicting queries to complete. This parameter can only be set in the `postgresql.conf` file or on the server command line.

Note that `max_standby_archive_delay` is not the same as the maximum length of time a query can run before cancellation; rather it is the maximum total time allowed to apply any one WAL segment's data. Thus, if one query has resulted in significant delay earlier in the WAL segment, subsequent conflicting queries will have much less grace time.
