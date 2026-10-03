---
name: "enable_indexonlyscan"
version: "18"
type: "boolean"
category: "Query Tuning / Planner Method Configuration"
short_desc: "Enables the planner's use of index-only-scan plans."
context: "user"
default: "on"
url: "https://www.postgresql.org/docs/18/runtime-config-query.html#GUC-ENABLE-INDEXONLYSCAN"
---

Enables or disables the query planner's use of index-only-scan plan types (see [Index-Only Scans and Covering Indexes](https://www.postgresql.org/docs/18/indexes-index-only-scans.html)). The default is `on`. The [`enable_indexscan`](https://www.postgresql.org/docs/18/runtime-config-query.html#GUC-ENABLE-INDEXSCAN) setting must also be enabled to have the query planner consider index-only-scans.
