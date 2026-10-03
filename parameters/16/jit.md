---
name: "jit"
version: "16"
type: "boolean"
category: "Query Tuning / Other Planner Options"
short_desc: "Allow JIT compilation."
context: "user"
default: "on"
url: "https://www.postgresql.org/docs/16/runtime-config-query.html#GUC-JIT"
---

Determines whether JIT compilation may be used by PostgreSQL, if available (see [Just-in-Time Compilation (JIT)](https://www.postgresql.org/docs/16/jit.html)). The default is `on`.
