---
name: "work_mem"
version: "9.3"
type: "integer"
category: "Resource Usage / Memory"
short_desc: "Sets the maximum memory to be used for query workspaces."
extra_desc: "This much memory can be used by each internal sort operation and hash table before switching to temporary disk files."
context: "user"
unit: "kB"
default: "1024"
min: "64"
max: "2147483647"
url: "https://www.postgresql.org/docs/9.3/runtime-config-resource.html#GUC-WORK-MEM"
---

Specifies the amount of memory to be used by internal sort operations and hash tables before writing to temporary disk files. The value defaults to one megabyte (`1MB`). Note that for a complex query, several sort or hash operations might be running in parallel; each operation will be allowed to use as much memory as this value specifies before it starts to write data into temporary files. Also, several running sessions could be doing such operations concurrently. Therefore, the total memory used could be many times the value of `work_mem`; it is necessary to keep this fact in mind when choosing the value. Sort operations are used for `ORDER BY`, `DISTINCT`, and merge joins. Hash tables are used in hash joins, hash-based aggregation, and hash-based processing of `IN` subqueries.
