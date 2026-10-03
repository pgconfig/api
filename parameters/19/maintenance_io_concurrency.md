---
name: "maintenance_io_concurrency"
version: "19"
type: "integer"
category: "Resource Usage / I/O"
short_desc: "A variant of \"effective_io_concurrency\" that is used for maintenance work."
extra_desc: "0 disables simultaneous requests."
context: "user"
default: "16"
min: "0"
max: "1000"
url: "https://www.postgresql.org/docs/19/runtime-config-resource.html#GUC-MAINTENANCE-IO-CONCURRENCY"
---

Similar to `effective_io_concurrency`, but used for maintenance work that is done on behalf of many client sessions.

The default is `16`. This value can be overridden for tables in a particular tablespace by setting the tablespace parameter of the same name (see [ALTER TABLESPACE](https://www.postgresql.org/docs/19/sql-altertablespace.html)).
