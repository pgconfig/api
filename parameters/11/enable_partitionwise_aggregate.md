---
name: "enable_partitionwise_aggregate"
version: "11"
type: "boolean"
category: "Query Tuning / Planner Method Configuration"
short_desc: "Enables partitionwise aggregation and grouping."
context: "user"
default: "off"
url: "https://www.postgresql.org/docs/11/runtime-config-query.html#GUC-ENABLE-PARTITIONWISE-AGGREGATE"
---

Enables or disables the query planner's use of partitionwise grouping or aggregation, which allows grouping or aggregation on partitioned tables to be performed separately for each partition. If the `GROUP BY` clause does not include the partition keys, only partial aggregation can be performed on a per-partition basis, and finalization must be performed later. Because partitionwise grouping or aggregation can use significantly more CPU time and memory during planning, the default is `off`.
