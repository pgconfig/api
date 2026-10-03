---
name: "max_sync_workers_per_subscription"
version: "19"
type: "integer"
category: "Replication / Subscribers"
short_desc: "Maximum number of workers per subscription for synchronizing tables and sequences."
context: "sighup"
default: "2"
min: "0"
max: "262143"
url: "https://www.postgresql.org/docs/19/runtime-config-replication.html#GUC-MAX-SYNC-WORKERS-PER-SUBSCRIPTION"
---

Maximum number of synchronization workers per subscription. This parameter controls the amount of parallelism of the initial data copy for tables during the subscription initialization or when new tables are added. One additional worker is also needed for sequence synchronization.

Currently, there can be only one table synchronization worker per table and one sequence synchronization worker to synchronize per subscription.

The synchronization workers are taken from the pool defined by `max_logical_replication_workers`.

The default value is 2. This parameter can only be set in the `postgresql.conf` file or on the server command line.
