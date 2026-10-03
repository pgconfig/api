---
name: "enable_seqscan"
version: "10"
type: "boolean"
category: "Query Tuning / Planner Method Configuration"
short_desc: "Enables the planner's use of sequential-scan plans."
context: "user"
default: "on"
url: "https://www.postgresql.org/docs/10/runtime-config-query.html#GUC-ENABLE-SEQSCAN"
---

Enables or disables the query planner's use of sequential scan plan types. It is impossible to suppress sequential scans entirely, but turning this variable off discourages the planner from using one if there are other methods available. The default is `on`.
