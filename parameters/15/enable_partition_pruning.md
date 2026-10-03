---
name: "enable_partition_pruning"
version: "15"
type: "boolean"
category: "Query Tuning / Planner Method Configuration"
short_desc: "Enables plan-time and execution-time partition pruning."
extra_desc: "Allows the query planner and executor to compare partition bounds to conditions in the query to determine which partitions must be scanned."
context: "user"
default: "on"
url: "https://www.postgresql.org/docs/15/runtime-config-query.html#GUC-ENABLE-PARTITION-PRUNING"
---

Enables or disables the query planner's ability to eliminate a partitioned table's partitions from query plans. This also controls the planner's ability to generate query plans which allow the query executor to remove (ignore) partitions during query execution. The default is `on`. See [Partition Pruning](https://www.postgresql.org/docs/15/ddl-partitioning.html#DDL-PARTITION-PRUNING) for details.
