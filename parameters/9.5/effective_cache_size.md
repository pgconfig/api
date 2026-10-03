---
name: "effective_cache_size"
version: "9.5"
type: "integer"
category: "Query Tuning / Planner Cost Constants"
short_desc: "Sets the planner's assumption about the total size of the data caches."
extra_desc: "That is, the total size of the caches (kernel cache and shared buffers) used for PostgreSQL data files. This is measured in disk pages, which are normally 8 kB each."
context: "user"
unit: "8kB"
default: "524288"
min: "1"
max: "2147483647"
url: "https://www.postgresql.org/docs/9.5/runtime-config-query.html#GUC-EFFECTIVE-CACHE-SIZE"
---

Sets the planner's assumption about the effective size of the disk cache that is available to a single query. This is factored into estimates of the cost of using an index; a higher value makes it more likely index scans will be used, a lower value makes it more likely sequential scans will be used. When setting this parameter you should consider both PostgreSQL's shared buffers and the portion of the kernel's disk cache that will be used for PostgreSQL data files, though some data might exist in both places. Also, take into account the expected number of concurrent queries on different tables, since they will have to share the available space. This parameter has no effect on the size of shared memory allocated by PostgreSQL, nor does it reserve kernel disk cache; it is used only for estimation purposes. The system also does not assume data remains in the disk cache between queries. The default is 4 gigabytes (`4GB`).
