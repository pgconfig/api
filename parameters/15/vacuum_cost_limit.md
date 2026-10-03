---
name: "vacuum_cost_limit"
version: "15"
type: "integer"
category: "Resource Usage / Cost-Based Vacuum Delay"
short_desc: "Vacuum cost amount available before napping."
context: "user"
default: "200"
min: "1"
max: "10000"
url: "https://www.postgresql.org/docs/15/runtime-config-resource.html#GUC-VACUUM-COST-LIMIT"
---

The accumulated cost that will cause the vacuuming process to sleep. The default value is 200.
