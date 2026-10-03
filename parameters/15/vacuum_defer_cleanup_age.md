---
name: "vacuum_defer_cleanup_age"
version: "15"
type: "integer"
category: "Replication / Primary Server"
short_desc: "Number of transactions by which VACUUM and HOT cleanup should be deferred, if any."
context: "sighup"
default: "0"
min: "0"
max: "1000000"
url: "https://www.postgresql.org/docs/15/runtime-config-replication.html#GUC-VACUUM-DEFER-CLEANUP-AGE"
---

Specifies the number of transactions by which `VACUUM` and [HOT updates](https://www.postgresql.org/docs/15/storage-hot.html) will defer cleanup of dead row versions. The default is zero transactions, meaning that dead row versions can be removed as soon as possible, that is, as soon as they are no longer visible to any open transaction. You may wish to set this to a non-zero value on a primary server that is supporting hot standby servers, as described in [Hot Standby](https://www.postgresql.org/docs/15/hot-standby.html). This allows more time for queries on the standby to complete without incurring conflicts due to early cleanup of rows. However, since the value is measured in terms of number of write transactions occurring on the primary server, it is difficult to predict just how much additional grace time will be made available to standby queries. This parameter can only be set in the `postgresql.conf` file or on the server command line.

You should also consider setting `hot_standby_feedback` on standby server(s) as an alternative to using this parameter.

This does not prevent cleanup of dead rows which have reached the age specified by `old_snapshot_threshold`.
