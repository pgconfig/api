---
name: "synchronous_standby_names"
version: "9.6"
type: "string"
category: "Replication / Master Server"
short_desc: "Number of synchronous standbys and list of names of potential synchronous ones."
context: "sighup"
default: ""
url: "https://www.postgresql.org/docs/9.6/runtime-config-replication.html#GUC-SYNCHRONOUS-STANDBY-NAMES"
---

Specifies a list of standby servers that can support *synchronous replication*, as described in [Synchronous Replication](https://www.postgresql.org/docs/9.6/warm-standby.html#SYNCHRONOUS-REPLICATION). There will be one or more active synchronous standbys; transactions waiting for commit will be allowed to proceed after these standby servers confirm receipt of their data. The synchronous standbys will be those whose names appear earlier in this list, and that are both currently connected and streaming data in real-time (as shown by a state of `streaming` in the [`pg_stat_replication`](https://www.postgresql.org/docs/9.6/monitoring-stats.html#PG-STAT-REPLICATION-VIEW) view). Other standby servers appearing later in this list represent potential synchronous standbys. If any of the current synchronous standbys disconnects for whatever reason, it will be replaced immediately with the next-highest-priority standby. Specifying more than one standby name can allow very high availability.

This parameter specifies a list of standby servers using either of the following syntaxes:

```
num_sync ( standby_name [, ...] )
standby_name [, ...]
```

where *num_sync* is the number of synchronous standbys that transactions need to wait for replies from, and *standby_name* is the name of a standby server. For example, a setting of `3 (s1, s2, s3, s4)` makes transaction commits wait until their WAL records are received by three higher-priority standbys chosen from standby servers `s1`, `s2`, `s3` and `s4`.

The second syntax was used before PostgreSQL version 9.6 and is still supported. It's the same as the first syntax with *num_sync* equal to 1. For example, `1 (s1, s2)` and `s1, s2` have the same meaning: either `s1` or `s2` is chosen as a synchronous standby.

The name of a standby server for this purpose is the `application_name` setting of the standby, as set in the `primary_conninfo` of the standby's WAL receiver. There is no mechanism to enforce uniqueness. In case of duplicates one of the matching standbys will be considered as higher priority, though exactly which one is indeterminate. The special entry `*` matches any `application_name`, including the default application name of `walreceiver`.

> [!NOTE]
> Each *standby_name* should have the form of a valid SQL identifier, unless it is `*`. You can use double-quoting if necessary. But note that *standby_name*s are compared to standby application names case-insensitively, whether double-quoted or not.

If no synchronous standby names are specified here, then synchronous replication is not enabled and transaction commits will not wait for replication. This is the default configuration. Even when synchronous replication is enabled, individual transactions can be configured not to wait for replication by setting the [`synchronous_commit`](https://www.postgresql.org/docs/9.6/runtime-config-wal.html#GUC-SYNCHRONOUS-COMMIT) parameter to `local` or `off`.

This parameter can only be set in the `postgresql.conf` file or on the server command line.
