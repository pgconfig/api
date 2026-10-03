---
name: "enable_parallel_append"
version: "12"
type: "boolean"
category: "Query Tuning / Planner Method Configuration"
short_desc: "Enables the planner's use of parallel append plans."
context: "user"
default: "on"
url: "https://www.postgresql.org/docs/12/runtime-config-query.html#GUC-ENABLE-PARALLEL-APPEND"
---

Enables or disables the query planner's use of parallel-aware append plan types. The default is `on`.
