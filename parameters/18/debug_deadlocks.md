---
name: "debug_deadlocks"
version: "18"
type: "boolean"
url: "https://www.postgresql.org/docs/18/runtime-config-developer.html#GUC-DEBUG-DEADLOCKS"
---

If set, dumps information about all current locks when a deadlock timeout occurs.

This parameter is only available if the `LOCK_DEBUG` macro was defined when PostgreSQL was compiled.
