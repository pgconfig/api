---
name: "maintenance_io_concurrency"
version: "16"
type: "integer"
category: "Resource Usage / Asynchronous Behavior"
short_desc: "A variant of effective_io_concurrency that is used for maintenance work."
context: "user"
default: "10"
min: "0"
max: "1000"
url: "https://www.postgresql.org/docs/16/runtime-config-resource.html#GUC-MAINTENANCE-IO-CONCURRENCY"
---

Similar to `effective_io_concurrency`, but used for maintenance work that is done on behalf of many client sessions.

The default is 10 on supported systems, otherwise 0. This value can be overridden for tables in a particular tablespace by setting the tablespace parameter of the same name (see [ALTER TABLESPACE](https://www.postgresql.org/docs/16/sql-altertablespace.html)).
