---
name: "enable_sort"
version: "18"
type: "boolean"
category: "Query Tuning / Planner Method Configuration"
short_desc: "Enables the planner's use of explicit sort steps."
context: "user"
default: "on"
url: "https://www.postgresql.org/docs/18/runtime-config-query.html#GUC-ENABLE-SORT"
---

Enables or disables the query planner's use of explicit sort steps. It is impossible to suppress explicit sorts entirely, but turning this variable off discourages the planner from using one if there are other methods available. The default is `on`.
