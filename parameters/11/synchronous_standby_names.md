---
name: "synchronous_standby_names"
version: "11"
type: "string"
category: "Replication / Master Server"
short_desc: "Number of synchronous standbys and list of names of potential synchronous ones."
context: "sighup"
default: ""
url: "https://www.postgresql.org/docs/11/runtime-config-replication.html#GUC-SYNCHRONOUS-STANDBY-NAMES"
---

Specifies a list of standby servers that can support *synchronous replication*, as described in [Synchronous Replication](https://www.postgresql.org/docs/11/warm-standby.html#SYNCHRONOUS-REPLICATION). There will be one or more active synchronous standbys; transactions waiting for commit will be allowed to proceed after these standby servers confirm receipt of their data. The synchronous standbys will be those whose names appear in this list, and that are both currently connected and streaming data in real-time (as shown by a state of `streaming` in the [`pg_stat_replication`](https://www.postgresql.org/docs/11/monitoring-stats.html#PG-STAT-REPLICATION-VIEW) view). Specifying more than one synchronous standby can allow for very high availability and protection against data loss.

The name of a standby server for this purpose is the `application_name` setting of the standby, as set in the standby's connection information. In case of a physical replication standby, this should be set in the `primary_conninfo` setting in `recovery.conf`; the default is `walreceiver`. For logical replication, this can be set in the connection information of the subscription, and it defaults to the subscription name. For other replication stream consumers, consult their documentation.

This parameter specifies a list of standby servers using either of the following syntaxes:

```
[FIRST] num_sync ( standby_name [, ...] )
ANY num_sync ( standby_name [, ...] )
standby_name [, ...]
```

where *num_sync* is the number of synchronous standbys that transactions need to wait for replies from, and *standby_name* is the name of a standby server. `FIRST` and `ANY` specify the method to choose synchronous standbys from the listed servers.

The keyword `FIRST`, coupled with *num_sync*, specifies a priority-based synchronous replication and makes transaction commits wait until their WAL records are replicated to *num_sync* synchronous standbys chosen based on their priorities. For example, a setting of `FIRST 3 (s1, s2, s3, s4)` will cause each commit to wait for replies from three higher-priority standbys chosen from standby servers `s1`, `s2`, `s3` and `s4`. The standbys whose names appear earlier in the list are given higher priority and will be considered as synchronous. Other standby servers appearing later in this list represent potential synchronous standbys. If any of the current synchronous standbys disconnects for whatever reason, it will be replaced immediately with the next-highest-priority standby. The keyword `FIRST` is optional.

The keyword `ANY`, coupled with *num_sync*, specifies a quorum-based synchronous replication and makes transaction commits wait until their WAL records are replicated to *at least* *num_sync* listed standbys. For example, a setting of `ANY 3 (s1, s2, s3, s4)` will cause each commit to proceed as soon as at least any three standbys of `s1`, `s2`, `s3` and `s4` reply.

`FIRST` and `ANY` are case-insensitive. If these keywords are used as the name of a standby server, its *standby_name* must be double-quoted.

The third syntax was used before PostgreSQL version 9.6 and is still supported. It's the same as the first syntax with `FIRST` and *num_sync* equal to 1. For example, `FIRST 1 (s1, s2)` and `s1, s2` have the same meaning: either `s1` or `s2` is chosen as a synchronous standby.

The special entry `*` matches any standby name.

There is no mechanism to enforce uniqueness of standby names. In case of duplicates one of the matching standbys will be considered as higher priority, though exactly which one is indeterminate.

> [!NOTE]
> Each *standby_name* should have the form of a valid SQL identifier, unless it is `*`. You can use double-quoting if necessary. But note that *standby_name*s are compared to standby application names case-insensitively, whether double-quoted or not.

If no synchronous standby names are specified here, then synchronous replication is not enabled and transaction commits will not wait for replication. This is the default configuration. Even when synchronous replication is enabled, individual transactions can be configured not to wait for replication by setting the [`synchronous_commit`](https://www.postgresql.org/docs/11/runtime-config-wal.html#GUC-SYNCHRONOUS-COMMIT) parameter to `local` or `off`.

This parameter can only be set in the `postgresql.conf` file or on the server command line.
