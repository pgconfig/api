---
name: "min_parallel_index_scan_size"
version: "12"
type: "integer"
category: "Query Tuning / Planner Cost Constants"
short_desc: "Sets the minimum amount of index data for a parallel scan."
extra_desc: "If the planner estimates that it will read a number of index pages too small to reach this limit, a parallel scan will not be considered."
context: "user"
unit: "8kB"
default: "64"
min: "0"
max: "715827882"
url: "https://www.postgresql.org/docs/12/runtime-config-query.html#GUC-MIN-PARALLEL-INDEX-SCAN-SIZE"
---

Sets the minimum amount of index data that must be scanned in order for a parallel scan to be considered. Note that a parallel index scan typically won't touch the entire index; it is the number of pages which the planner believes will actually be touched by the scan which is relevant. If this value is specified without units, it is taken as blocks, that is `BLCKSZ` bytes, typically 8kB. The default is 512 kilobytes (`512kB`).
