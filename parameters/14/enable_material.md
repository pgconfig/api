---
name: "enable_material"
version: "14"
type: "boolean"
category: "Query Tuning / Planner Method Configuration"
short_desc: "Enables the planner's use of materialization."
context: "user"
default: "on"
url: "https://www.postgresql.org/docs/14/runtime-config-query.html#GUC-ENABLE-MATERIAL"
---

Enables or disables the query planner's use of materialization. It is impossible to suppress materialization entirely, but turning this variable off prevents the planner from inserting materialize nodes except in cases where it is required for correctness. The default is `on`.
