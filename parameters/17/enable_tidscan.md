---
name: "enable_tidscan"
version: "17"
type: "boolean"
category: "Query Tuning / Planner Method Configuration"
short_desc: "Enables the planner's use of TID scan plans."
context: "user"
default: "on"
url: "https://www.postgresql.org/docs/17/runtime-config-query.html#GUC-ENABLE-TIDSCAN"
---

Enables or disables the query planner's use of TID scan plan types. The default is `on`.
