---
name: "min_parallel_table_scan_size"
version: "16"
type: "integer"
category: "Query Tuning / Planner Cost Constants"
short_desc: "Sets the minimum amount of table data for a parallel scan."
extra_desc: "If the planner estimates that it will read a number of table pages too small to reach this limit, a parallel scan will not be considered."
context: "user"
unit: "8kB"
default: "1024"
min: "0"
max: "715827882"
url: "https://www.postgresql.org/docs/16/runtime-config-query.html#GUC-MIN-PARALLEL-TABLE-SCAN-SIZE"
---

Sets the minimum amount of table data that must be scanned in order for a parallel scan to be considered. For a parallel sequential scan, the amount of table data scanned is always equal to the size of the table, but when indexes are used the amount of table data scanned will normally be less. If this value is specified without units, it is taken as blocks, that is `BLCKSZ` bytes, typically 8kB. The default is 8 megabytes (`8MB`).
