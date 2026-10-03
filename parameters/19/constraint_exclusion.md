---
name: "constraint_exclusion"
version: "19"
type: "enum"
category: "Query Tuning / Other Planner Options"
short_desc: "Enables the planner to use constraints to optimize queries."
extra_desc: "Table scans will be skipped if their constraints guarantee that no rows match the query."
context: "user"
default: "partition"
values: ["partition", "on", "off"]
url: "https://www.postgresql.org/docs/19/runtime-config-query.html#GUC-CONSTRAINT-EXCLUSION"
---

Controls the query planner's use of table constraints to optimize queries. The allowed values of `constraint_exclusion` are `on` (examine constraints for all tables), `off` (never examine constraints), and `partition` (examine constraints only for inheritance child tables and `UNION ALL` subqueries). `partition` is the default setting. It is often used with traditional inheritance trees to improve performance.

When this parameter allows it for a particular table, the planner compares query conditions with the table's `CHECK` constraints, and omits scanning tables for which the conditions contradict the constraints. For example:

```
CREATE TABLE parent(key integer, ...);
CREATE TABLE child1000(CHECK (key BETWEEN 1000 AND 1999)) INHERITS(parent);
CREATE TABLE child2000(CHECK (key BETWEEN 2000 AND 2999)) INHERITS(parent);
...
SELECT * FROM parent WHERE key = 2400;
```

With constraint exclusion enabled, this `SELECT` will not scan `child1000` at all, improving performance.

Currently, constraint exclusion is enabled by default only for cases that are often used to implement table partitioning via inheritance trees. Turning it on for all tables imposes extra planning overhead that is quite noticeable on simple queries, and most often will yield no benefit for simple queries. If you have no tables that are partitioned using traditional inheritance, you might prefer to turn it off entirely. (Note that the equivalent feature for partitioned tables is controlled by a separate parameter, [`enable_partition_pruning`](https://www.postgresql.org/docs/19/runtime-config-query.html#GUC-ENABLE-PARTITION-PRUNING).)

Refer to [Partitioning and Constraint Exclusion](https://www.postgresql.org/docs/19/ddl-partitioning.html#DDL-PARTITIONING-CONSTRAINT-EXCLUSION) for more information on using constraint exclusion to implement partitioning.
