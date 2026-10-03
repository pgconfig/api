---
name: "trace_lwlocks"
version: "17"
type: "boolean"
url: "https://www.postgresql.org/docs/17/runtime-config-developer.html#GUC-TRACE-LWLOCKS"
---

If on, emit information about lightweight lock usage. Lightweight locks are intended primarily to provide mutual exclusion of access to shared-memory data structures.

This parameter is only available if the `LOCK_DEBUG` macro was defined when PostgreSQL was compiled.
