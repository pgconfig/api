---
name: "enable_tidscan"
version: "14"
type: "boolean"
category: "Query Tuning / Planner Method Configuration"
short_desc: "Enables the planner's use of TID scan plans."
context: "user"
default: "on"
url: "https://www.postgresql.org/docs/14/runtime-config-query.html#GUC-ENABLE-TIDSCAN"
---

Enables or disables the query planner's use of TID scan plan types. The default is `on`.
