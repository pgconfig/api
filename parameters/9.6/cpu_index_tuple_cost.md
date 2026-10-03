---
name: "cpu_index_tuple_cost"
version: "9.6"
type: "floating point"
category: "Query Tuning / Planner Cost Constants"
short_desc: "Sets the planner's estimate of the cost of processing each index entry during an index scan."
context: "user"
default: "0.005"
min: "0"
max: "1.79769e+308"
url: "https://www.postgresql.org/docs/9.6/runtime-config-query.html#GUC-CPU-INDEX-TUPLE-COST"
---

Sets the planner's estimate of the cost of processing each index entry during an index scan. The default is 0.005.
