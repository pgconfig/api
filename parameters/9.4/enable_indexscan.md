---
name: "enable_indexscan"
version: "9.4"
type: "boolean"
category: "Query Tuning / Planner Method Configuration"
short_desc: "Enables the planner's use of index-scan plans."
context: "user"
default: "on"
url: "https://www.postgresql.org/docs/9.4/runtime-config-query.html#GUC-ENABLE-INDEXSCAN"
---

Enables or disables the query planner's use of index-scan plan types. The default is `on`.
