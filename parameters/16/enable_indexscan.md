---
name: "enable_indexscan"
version: "16"
type: "boolean"
category: "Query Tuning / Planner Method Configuration"
short_desc: "Enables the planner's use of index-scan plans."
context: "user"
default: "on"
url: "https://www.postgresql.org/docs/16/runtime-config-query.html#GUC-ENABLE-INDEXSCAN"
---

Enables or disables the query planner's use of index-scan and index-only-scan plan types. The default is `on`. Also see [`enable_indexonlyscan`](https://www.postgresql.org/docs/16/runtime-config-query.html#GUC-ENABLE-INDEXONLYSCAN).
