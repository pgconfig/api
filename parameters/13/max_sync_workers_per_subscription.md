---
name: "max_sync_workers_per_subscription"
version: "13"
type: "integer"
category: "Replication / Subscribers"
short_desc: "Maximum number of table synchronization workers per subscription."
context: "sighup"
default: "2"
min: "0"
max: "262143"
url: "https://www.postgresql.org/docs/13/runtime-config-replication.html#GUC-MAX-SYNC-WORKERS-PER-SUBSCRIPTION"
---

Maximum number of synchronization workers per subscription. This parameter controls the amount of parallelism of the initial data copy during the subscription initialization or when new tables are added.

Currently, there can be only one synchronization worker per table.

The synchronization workers are taken from the pool defined by `max_logical_replication_workers`.

The default value is 2. This parameter can only be set in the `postgresql.conf` file or on the server command line.
