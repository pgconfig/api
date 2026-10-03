---
name: "jit_above_cost"
version: "17"
type: "floating point"
category: "Query Tuning / Planner Cost Constants"
short_desc: "Perform JIT compilation if query is more expensive."
extra_desc: "-1 disables JIT compilation."
context: "user"
default: "100000"
min: "-1"
max: "1.79769e+308"
url: "https://www.postgresql.org/docs/17/runtime-config-query.html#GUC-JIT-ABOVE-COST"
---

Sets the query cost above which JIT compilation is activated, if enabled (see [Just-in-Time Compilation (JIT)](https://www.postgresql.org/docs/17/jit.html)). Performing JIT costs planning time but can accelerate query execution. Setting this to `-1` disables JIT compilation. The default is `100000`.
