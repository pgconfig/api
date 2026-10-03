---
name: "wal_receiver_status_interval"
version: "11"
type: "integer"
category: "Replication / Standby Servers"
short_desc: "Sets the maximum interval between WAL receiver status reports to the primary."
context: "sighup"
unit: "s"
default: "10"
min: "0"
max: "2147483"
url: "https://www.postgresql.org/docs/11/runtime-config-replication.html#GUC-WAL-RECEIVER-STATUS-INTERVAL"
---

Specifies the minimum frequency for the WAL receiver process on the standby to send information about replication progress to the primary or upstream standby, where it can be seen using the [`pg_stat_replication`](https://www.postgresql.org/docs/11/monitoring-stats.html#PG-STAT-REPLICATION-VIEW) view. The standby will report the last write-ahead log location it has written, the last position it has flushed to disk, and the last position it has applied. This parameter's value is the maximum interval, in seconds, between reports. Updates are sent each time the write or flush positions change, or at least as often as specified by this parameter. Thus, the apply position may lag slightly behind the true position. Setting this parameter to zero disables status updates completely. This parameter can only be set in the `postgresql.conf` file or on the server command line. The default value is 10 seconds.
