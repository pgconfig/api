---
name: "enable_indexonlyscan"
version: "11"
type: "boolean"
category: "Query Tuning / Planner Method Configuration"
short_desc: "Enables the planner's use of index-only-scan plans."
context: "user"
default: "on"
url: "https://www.postgresql.org/docs/11/runtime-config-query.html#GUC-ENABLE-INDEXONLYSCAN"
---

Enables or disables the query planner's use of index-only-scan plan types (see [Index-Only Scans and Covering Indexes](https://www.postgresql.org/docs/11/indexes-index-only-scans.html)). The default is `on`.
