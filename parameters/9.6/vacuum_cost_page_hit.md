---
name: "vacuum_cost_page_hit"
version: "9.6"
type: "integer"
category: "Resource Usage / Cost-Based Vacuum Delay"
short_desc: "Vacuum cost for a page found in the buffer cache."
context: "user"
default: "1"
min: "0"
max: "10000"
url: "https://www.postgresql.org/docs/9.6/runtime-config-resource.html#GUC-VACUUM-COST-PAGE-HIT"
---

The estimated cost for vacuuming a buffer found in the shared buffer cache. It represents the cost to lock the buffer pool, lookup the shared hash table and scan the content of the page. The default value is one.
