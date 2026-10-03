---
name: "vacuum_cost_delay"
version: "9.5"
type: "integer"
category: "Resource Usage / Cost-Based Vacuum Delay"
short_desc: "Vacuum cost delay in milliseconds."
context: "user"
unit: "ms"
default: "0"
min: "0"
max: "100"
url: "https://www.postgresql.org/docs/9.5/runtime-config-resource.html#GUC-VACUUM-COST-DELAY"
---

The length of time, in milliseconds, that the process will sleep when the cost limit has been exceeded. The default value is zero, which disables the cost-based vacuum delay feature. Positive values enable cost-based vacuuming. Note that on many systems, the effective resolution of sleep delays is 10 milliseconds; setting `vacuum_cost_delay` to a value that is not a multiple of 10 might have the same results as setting it to the next higher multiple of 10.

When using cost-based vacuuming, appropriate values for `vacuum_cost_delay` are usually quite small, perhaps 10 or 20 milliseconds. Adjusting vacuum's resource consumption is best done by changing the other vacuum cost parameters.
