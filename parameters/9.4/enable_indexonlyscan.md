---
name: "enable_indexonlyscan"
version: "9.4"
type: "boolean"
category: "Query Tuning / Planner Method Configuration"
short_desc: "Enables the planner's use of index-only-scan plans."
context: "user"
default: "on"
url: "https://www.postgresql.org/docs/9.4/runtime-config-query.html#GUC-ENABLE-INDEXONLYSCAN"
---

Enables or disables the query planner's use of index-only-scan plan types. The default is `on`.
