---
name: "enable_async_append"
version: "15"
type: "boolean"
category: "Query Tuning / Planner Method Configuration"
short_desc: "Enables the planner's use of async append plans."
context: "user"
default: "on"
url: "https://www.postgresql.org/docs/15/runtime-config-query.html#GUC-ENABLE-ASYNC-APPEND"
---

Enables or disables the query planner's use of async-aware append plan types. The default is `on`.
