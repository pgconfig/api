---
name: "enable_gathermerge"
version: "15"
type: "boolean"
category: "Query Tuning / Planner Method Configuration"
short_desc: "Enables the planner's use of gather merge plans."
context: "user"
default: "on"
url: "https://www.postgresql.org/docs/15/runtime-config-query.html#GUC-ENABLE-GATHERMERGE"
---

Enables or disables the query planner's use of gather merge plan types. The default is `on`.
