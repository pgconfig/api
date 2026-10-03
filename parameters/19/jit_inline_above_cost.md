---
name: "jit_inline_above_cost"
version: "19"
type: "floating point"
category: "Query Tuning / Planner Cost Constants"
short_desc: "Perform JIT inlining if query is more expensive."
extra_desc: "-1 disables inlining."
context: "user"
default: "500000"
min: "-1"
max: "1.79769e+308"
url: "https://www.postgresql.org/docs/19/runtime-config-query.html#GUC-JIT-INLINE-ABOVE-COST"
---

Sets the query cost above which JIT compilation attempts to inline functions and operators. Inlining adds planning time, but can improve execution speed. It is not meaningful to set this to less than `jit_above_cost`. Setting this to `-1` disables inlining. The default is `500000`.
