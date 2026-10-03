---
name: "jit_optimize_above_cost"
version: "14"
type: "floating point"
category: "Query Tuning / Planner Cost Constants"
short_desc: "Optimize JIT-compiled functions if query is more expensive."
extra_desc: "-1 disables optimization."
context: "user"
default: "500000"
min: "-1"
max: "1.79769e+308"
url: "https://www.postgresql.org/docs/14/runtime-config-query.html#GUC-JIT-OPTIMIZE-ABOVE-COST"
---

Sets the query cost above which JIT compilation applies expensive optimizations. Such optimization adds planning time, but can improve execution speed. It is not meaningful to set this to less than `jit_above_cost`, and it is unlikely to be beneficial to set it to more than `jit_inline_above_cost`. Setting this to `-1` disables expensive optimizations. The default is `500000`.
