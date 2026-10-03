---
name: "enable_parallel_hash"
version: "15"
type: "boolean"
category: "Query Tuning / Planner Method Configuration"
short_desc: "Enables the planner's use of parallel hash plans."
context: "user"
default: "on"
url: "https://www.postgresql.org/docs/15/runtime-config-query.html#GUC-ENABLE-PARALLEL-HASH"
---

Enables or disables the query planner's use of hash-join plan types with parallel hash. Has no effect if hash-join plans are not also enabled. The default is `on`.
