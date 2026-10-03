---
name: "enable_presorted_aggregate"
version: "17"
type: "boolean"
category: "Query Tuning / Planner Method Configuration"
short_desc: "Enables the planner's ability to produce plans that provide presorted input for ORDER BY / DISTINCT aggregate functions."
extra_desc: "Allows the query planner to build plans that provide presorted input for aggregate functions with an ORDER BY / DISTINCT clause.  When disabled, implicit sorts are always performed during execution."
context: "user"
default: "on"
url: "https://www.postgresql.org/docs/17/runtime-config-query.html#GUC-ENABLE-PRESORTED-AGGREGATE"
---

Controls if the query planner will produce a plan which will provide rows which are presorted in the order required for the query's `ORDER BY` / `DISTINCT` aggregate functions. When disabled, the query planner will produce a plan which will always require the executor to perform a sort before performing aggregation of each aggregate function containing an `ORDER BY` or `DISTINCT` clause. When enabled, the planner will try to produce a more efficient plan which provides input to the aggregate functions which is presorted in the order they require for aggregation. The default value is `on`.
