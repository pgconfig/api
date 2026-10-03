---
name: "enable_nestloop"
version: "9.4"
type: "boolean"
category: "Query Tuning / Planner Method Configuration"
short_desc: "Enables the planner's use of nested-loop join plans."
context: "user"
default: "on"
url: "https://www.postgresql.org/docs/9.4/runtime-config-query.html#GUC-ENABLE-NESTLOOP"
---

Enables or disables the query planner's use of nested-loop join plans. It is impossible to suppress nested-loop joins entirely, but turning this variable off discourages the planner from using one if there are other methods available. The default is `on`.
