---
name: "constraint_exclusion"
version: "9.6"
type: "enum"
category: "Query Tuning / Other Planner Options"
short_desc: "Enables the planner to use constraints to optimize queries."
extra_desc: "Table scans will be skipped if their constraints guarantee that no rows match the query."
context: "user"
default: "partition"
values: ["partition", "on", "off"]
url: "https://www.postgresql.org/docs/9.6/runtime-config-query.html#GUC-CONSTRAINT-EXCLUSION"
---

Controls the query planner's use of table constraints to optimize queries. The allowed values of `constraint_exclusion` are `on` (examine constraints for all tables), `off` (never examine constraints), and `partition` (examine constraints only for inheritance child tables and `UNION ALL` subqueries). `partition` is the default setting. It is often used with inheritance and partitioned tables to improve performance.

When this parameter allows it for a particular table, the planner compares query conditions with the table's `CHECK` constraints, and omits scanning tables for which the conditions contradict the constraints. For example:

```
CREATE TABLE parent(key integer, ...);
CREATE TABLE child1000(check (key between 1000 and 1999)) INHERITS(parent);
CREATE TABLE child2000(check (key between 2000 and 2999)) INHERITS(parent);
...
SELECT * FROM parent WHERE key = 2400;
```

With constraint exclusion enabled, this `SELECT` will not scan `child1000` at all, improving performance.

Currently, constraint exclusion is enabled by default only for cases that are often used to implement table partitioning. Turning it on for all tables imposes extra planning overhead that is quite noticeable on simple queries, and most often will yield no benefit for simple queries. If you have no partitioned tables you might prefer to turn it off entirely.

Refer to [Partitioning and Constraint Exclusion](https://www.postgresql.org/docs/9.6/ddl-partitioning.html#DDL-PARTITIONING-CONSTRAINT-EXCLUSION) for more information on using constraint exclusion and partitioning.
