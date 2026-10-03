---
name: "enable_hashagg"
version: "19"
type: "boolean"
category: "Query Tuning / Planner Method Configuration"
short_desc: "Enables the planner's use of hashed aggregation plans."
context: "user"
default: "on"
url: "https://www.postgresql.org/docs/19/runtime-config-query.html#GUC-ENABLE-HASHAGG"
---

Enables or disables the query planner's use of hashed aggregation plan types. The default is `on`.
