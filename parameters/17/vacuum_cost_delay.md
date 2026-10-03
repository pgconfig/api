---
name: "vacuum_cost_delay"
version: "17"
type: "floating point"
category: "Resource Usage / Cost-Based Vacuum Delay"
short_desc: "Vacuum cost delay in milliseconds."
context: "user"
unit: "ms"
default: "0"
min: "0"
max: "100"
url: "https://www.postgresql.org/docs/17/runtime-config-resource.html#GUC-VACUUM-COST-DELAY"
---

The amount of time that the process will sleep when the cost limit has been exceeded. If this value is specified without units, it is taken as milliseconds. The default value is zero, which disables the cost-based vacuum delay feature. Positive values enable cost-based vacuuming.

When using cost-based vacuuming, appropriate values for `vacuum_cost_delay` are usually quite small, perhaps less than 1 millisecond. While `vacuum_cost_delay` can be set to fractional-millisecond values, such delays may not be measured accurately on older platforms. On such platforms, increasing `VACUUM`'s throttled resource consumption above what you get at 1ms will require changing the other vacuum cost parameters. You should, nonetheless, keep `vacuum_cost_delay` as small as your platform will consistently measure; large delays are not helpful.
