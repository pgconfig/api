---
name: "enable_eager_aggregate"
version: "19"
type: "boolean"
category: "Query Tuning / Planner Method Configuration"
short_desc: "Enables eager aggregation."
context: "user"
default: "on"
url: "https://www.postgresql.org/docs/19/runtime-config-query.html#GUC-ENABLE-EAGER-AGGREGATE"
---

Enables or disables the query planner's ability to partially push aggregation past a join, and finalize it once all the relations are joined. The default is `on`.
