---
name: "wal_retrieve_retry_interval"
version: "18"
type: "integer"
category: "Replication / Standby Servers"
short_desc: "Sets the time to wait before retrying to retrieve WAL after a failed attempt."
context: "sighup"
unit: "ms"
default: "5000"
min: "1"
max: "2147483647"
url: "https://www.postgresql.org/docs/18/runtime-config-replication.html#GUC-WAL-RETRIEVE-RETRY-INTERVAL"
---

Specifies how long the standby server should wait when WAL data is not available from any sources (streaming replication, local `pg_wal` or WAL archive) before trying again to retrieve WAL data. If this value is specified without units, it is taken as milliseconds. The default value is 5 seconds. This parameter can only be set in the `postgresql.conf` file or on the server command line.

This parameter is useful in configurations where a node in recovery needs to control the amount of time to wait for new WAL data to be available. For example, in archive recovery, it is possible to make the recovery more responsive in the detection of a new WAL file by reducing the value of this parameter. On a system with low WAL activity, increasing it reduces the amount of requests necessary to access WAL archives, something useful for example in cloud environments where the number of times an infrastructure is accessed is taken into account.

In logical replication, this parameter also limits how often a failing replication apply worker or table synchronization worker will be respawned.
