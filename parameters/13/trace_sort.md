---
name: "trace_sort"
version: "13"
type: "boolean"
category: "Developer Options"
short_desc: "Emit information about resource usage in sorting."
context: "user"
default: "off"
url: "https://www.postgresql.org/docs/13/runtime-config-developer.html#GUC-TRACE-SORT"
---

If on, emit information about resource usage during sort operations. This parameter is only available if the `TRACE_SORT` macro was defined when PostgreSQL was compiled. (However, `TRACE_SORT` is currently defined by default.)
