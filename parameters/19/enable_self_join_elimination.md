---
name: "enable_self_join_elimination"
version: "19"
type: "boolean"
category: "Query Tuning / Planner Method Configuration"
short_desc: "Enables removal of unique self-joins."
context: "user"
default: "on"
url: "https://www.postgresql.org/docs/19/runtime-config-query.html#GUC-ENABLE-SELF-JOIN-ELIMINATION"
---

Enables or disables the query planner's optimization which analyses the query tree and replaces self joins with semantically equivalent single scans. Takes into consideration only plain tables. The default is `on`.
