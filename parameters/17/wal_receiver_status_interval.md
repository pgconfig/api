---
name: "wal_receiver_status_interval"
version: "17"
type: "integer"
category: "Replication / Standby Servers"
short_desc: "Sets the maximum interval between WAL receiver status reports to the sending server."
context: "sighup"
unit: "s"
default: "10"
min: "0"
max: "2147483"
url: "https://www.postgresql.org/docs/17/runtime-config-replication.html#GUC-WAL-RECEIVER-STATUS-INTERVAL"
---

Specifies the minimum frequency for the WAL receiver process on the standby to send information about replication progress to the primary or upstream standby, where it can be seen using the [`pg_stat_replication`](https://www.postgresql.org/docs/17/monitoring-stats.html#MONITORING-PG-STAT-REPLICATION-VIEW) view. The standby will report the last write-ahead log location it has written, the last position it has flushed to disk, and the last position it has applied. This parameter's value is the maximum amount of time between reports. Updates are sent each time the write or flush positions change, or as often as specified by this parameter if set to a non-zero value. There are additional cases where updates are sent while ignoring this parameter; for example, when processing of the existing WAL completes or when `synchronous_commit` is set to `remote_apply`. Thus, the apply position may lag slightly behind the true position. If this value is specified without units, it is taken as seconds. The default value is 10 seconds. This parameter can only be set in the `postgresql.conf` file or on the server command line.
