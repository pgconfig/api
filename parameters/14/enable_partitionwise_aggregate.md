---
name: "enable_partitionwise_aggregate"
version: "14"
type: "boolean"
category: "Query Tuning / Planner Method Configuration"
short_desc: "Enables partitionwise aggregation and grouping."
context: "user"
default: "off"
url: "https://www.postgresql.org/docs/14/runtime-config-query.html#GUC-ENABLE-PARTITIONWISE-AGGREGATE"
---

Enables or disables the query planner's use of partitionwise grouping or aggregation, which allows grouping or aggregation on partitioned tables to be performed separately for each partition. If the `GROUP BY` clause does not include the partition keys, only partial aggregation can be performed on a per-partition basis, and finalization must be performed later. With this setting enabled, the number of nodes whose memory usage is restricted by `work_mem` appearing in the final plan can increase linearly according to the number of partitions being scanned. This can result in a large increase in overall memory consumption during the execution of the query. Query planning also becomes significantly more expensive in terms of memory and CPU. The default value is `off`.
