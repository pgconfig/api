---
name: "synchronous_standby_names"
version: "9.5"
type: "string"
category: "Replication / Master Server"
short_desc: "List of names of potential synchronous standbys."
context: "sighup"
default: ""
url: "https://www.postgresql.org/docs/9.5/runtime-config-replication.html#GUC-SYNCHRONOUS-STANDBY-NAMES"
---

Specifies a comma-separated list of standby names that can support *synchronous replication*, as described in [Synchronous Replication](https://www.postgresql.org/docs/9.5/warm-standby.html#SYNCHRONOUS-REPLICATION). At any one time there will be at most one active synchronous standby; transactions waiting for commit will be allowed to proceed after this standby server confirms receipt of their data. The synchronous standby will be the first standby named in this list that is both currently connected and streaming data in real-time (as shown by a state of `streaming` in the [`pg_stat_replication`](https://www.postgresql.org/docs/9.5/monitoring-stats.html#PG-STAT-REPLICATION-VIEW) view). Other standby servers appearing later in this list represent potential synchronous standbys. If the current synchronous standby disconnects for whatever reason, it will be replaced immediately with the next-highest-priority standby. Specifying more than one standby name can allow very high availability.

The name of a standby server for this purpose is the `application_name` setting of the standby, as set in the `primary_conninfo` of the standby's WAL receiver. There is no mechanism to enforce uniqueness. In case of duplicates one of the matching standbys will be chosen to be the synchronous standby, though exactly which one is indeterminate. The special entry `*` matches any `application_name`, including the default application name of `walreceiver`.

If no synchronous standby names are specified here, then synchronous replication is not enabled and transaction commits will not wait for replication. This is the default configuration. Even when synchronous replication is enabled, individual transactions can be configured not to wait for replication by setting the [`synchronous_commit`](https://www.postgresql.org/docs/9.5/runtime-config-wal.html#GUC-SYNCHRONOUS-COMMIT) parameter to `local` or `off`.

This parameter can only be set in the `postgresql.conf` file or on the server command line.
