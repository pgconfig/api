---
name: "enable_bitmapscan"
version: "13"
type: "boolean"
category: "Query Tuning / Planner Method Configuration"
short_desc: "Enables the planner's use of bitmap-scan plans."
context: "user"
default: "on"
url: "https://www.postgresql.org/docs/13/runtime-config-query.html#GUC-ENABLE-BITMAPSCAN"
---

Enables or disables the query planner's use of bitmap-scan plan types. The default is `on`.
