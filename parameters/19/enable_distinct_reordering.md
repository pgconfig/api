---
name: "enable_distinct_reordering"
version: "19"
type: "boolean"
category: "Query Tuning / Planner Method Configuration"
short_desc: "Enables reordering of DISTINCT keys."
context: "user"
default: "on"
url: "https://www.postgresql.org/docs/19/runtime-config-query.html#GUC-ENABLE-DISTINCT-REORDERING"
---

Enables or disables the query planner's ability to reorder DISTINCT keys to match the input path's pathkeys. The default is `on`.
