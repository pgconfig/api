---
name: "trace_userlocks"
version: "13"
type: "boolean"
url: "https://www.postgresql.org/docs/13/runtime-config-developer.html#GUC-TRACE-USERLOCKS"
---

If on, emit information about user lock usage. Output is the same as for `trace_locks`, only for advisory locks.

This parameter is only available if the `LOCK_DEBUG` macro was defined when PostgreSQL was compiled.
