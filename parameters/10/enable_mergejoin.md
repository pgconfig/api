---
name: "enable_mergejoin"
version: "10"
type: "boolean"
category: "Query Tuning / Planner Method Configuration"
short_desc: "Enables the planner's use of merge join plans."
context: "user"
default: "on"
url: "https://www.postgresql.org/docs/10/runtime-config-query.html#GUC-ENABLE-MERGEJOIN"
---

Enables or disables the query planner's use of merge-join plan types. The default is `on`.
