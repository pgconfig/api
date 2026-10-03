---
name: "vacuum_cost_page_dirty"
version: "15"
type: "integer"
category: "Resource Usage / Cost-Based Vacuum Delay"
short_desc: "Vacuum cost for a page dirtied by vacuum."
context: "user"
default: "20"
min: "0"
max: "10000"
url: "https://www.postgresql.org/docs/15/runtime-config-resource.html#GUC-VACUUM-COST-PAGE-DIRTY"
---

The estimated cost charged when vacuum modifies a block that was previously clean. It represents the extra I/O required to flush the dirty block out to disk again. The default value is 20.
