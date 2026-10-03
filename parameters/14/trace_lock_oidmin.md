---
name: "trace_lock_oidmin"
version: "14"
type: "integer"
url: "https://www.postgresql.org/docs/14/runtime-config-developer.html#GUC-TRACE-LOCK-OIDMIN"
---

If set, do not trace locks for tables below this OID (used to avoid output on system tables).

This parameter is only available if the `LOCK_DEBUG` macro was defined when PostgreSQL was compiled.
