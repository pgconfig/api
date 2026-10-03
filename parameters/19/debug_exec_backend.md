---
name: "debug_exec_backend"
version: "19"
type: "boolean"
category: "Preset Options"
short_desc: "Shows whether the running server is built with EXEC_BACKEND enabled."
context: "internal"
default: "off"
url: "https://www.postgresql.org/docs/19/runtime-config-preset.html#GUC-DEBUG-EXEC-BACKEND"
---

Reports whether PostgreSQL has been built with `EXEC_BACKEND` enabled. That is the case on `Windows` or if the macro `EXEC_BACKEND` is defined when PostgreSQL is built.
