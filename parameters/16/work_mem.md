---
name: "work_mem"
version: "16"
type: "integer"
category: "Resource Usage / Memory"
short_desc: "Sets the maximum memory to be used for query workspaces."
extra_desc: "This much memory can be used by each internal sort operation and hash table before switching to temporary disk files."
context: "user"
unit: "kB"
default: "4096"
min: "64"
max: "2147483647"
url: "https://www.postgresql.org/docs/16/runtime-config-resource.html#GUC-WORK-MEM"
---

Sets the base maximum amount of memory to be used by a query operation (such as a sort or hash table) before writing to temporary disk files. If this value is specified without units, it is taken as kilobytes. The default value is four megabytes (`4MB`). Note that a complex query might perform several sort and hash operations at the same time, with each operation generally being allowed to use as much memory as this value specifies before it starts to write data into temporary files. Also, several running sessions could be doing such operations concurrently. Therefore, the total memory used could be many times the value of `work_mem`; it is necessary to keep this fact in mind when choosing the value. Sort operations are used for `ORDER BY`, `DISTINCT`, and merge joins. Hash tables are used in hash joins, hash-based aggregation, memoize nodes and hash-based processing of `IN` subqueries.

Hash-based operations are generally more sensitive to memory availability than equivalent sort-based operations. The memory limit for a hash table is computed by multiplying `work_mem` by `hash_mem_multiplier`. This makes it possible for hash-based operations to use an amount of memory that exceeds the usual `work_mem` base amount.
