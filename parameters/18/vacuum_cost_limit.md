---
name: "vacuum_cost_limit"
version: "18"
type: "integer"
category: "Vacuuming / Cost-Based Vacuum Delay"
short_desc: "Vacuum cost amount available before napping."
context: "user"
default: "200"
min: "1"
max: "10000"
url: "https://www.postgresql.org/docs/18/runtime-config-vacuum.html#GUC-VACUUM-COST-LIMIT"
---

This is the accumulated cost that will cause the vacuuming process to sleep for `vacuum_cost_delay`. The default is `200`.
