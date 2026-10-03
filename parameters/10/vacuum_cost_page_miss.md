---
name: "vacuum_cost_page_miss"
version: "10"
type: "integer"
category: "Resource Usage / Cost-Based Vacuum Delay"
short_desc: "Vacuum cost for a page not found in the buffer cache."
context: "user"
default: "10"
min: "0"
max: "10000"
url: "https://www.postgresql.org/docs/10/runtime-config-resource.html#GUC-VACUUM-COST-PAGE-MISS"
---

The estimated cost for vacuuming a buffer that has to be read from disk. This represents the effort to lock the buffer pool, lookup the shared hash table, read the desired block in from the disk and scan its content. The default value is 10.
