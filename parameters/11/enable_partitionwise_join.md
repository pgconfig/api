---
name: "enable_partitionwise_join"
version: "11"
type: "boolean"
category: "Query Tuning / Planner Method Configuration"
short_desc: "Enables partitionwise join."
context: "user"
default: "off"
url: "https://www.postgresql.org/docs/11/runtime-config-query.html#GUC-ENABLE-PARTITIONWISE-JOIN"
---

Enables or disables the query planner's use of partitionwise join, which allows a join between partitioned tables to be performed by joining the matching partitions. Partitionwise join currently applies only when the join conditions include all the partition keys, which must be of the same data type and have exactly matching sets of child partitions. Because partitionwise join planning can use significantly more CPU time and memory during planning, the default is `off`.
