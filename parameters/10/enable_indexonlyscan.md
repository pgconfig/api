---
name: "enable_indexonlyscan"
version: "10"
type: "boolean"
category: "Query Tuning / Planner Method Configuration"
short_desc: "Enables the planner's use of index-only-scan plans."
context: "user"
default: "on"
url: "https://www.postgresql.org/docs/10/runtime-config-query.html#GUC-ENABLE-INDEXONLYSCAN"
---

Enables or disables the query planner's use of index-only-scan plan types (see [Index-Only Scans](https://www.postgresql.org/docs/10/indexes-index-only-scans.html)). The default is `on`.
