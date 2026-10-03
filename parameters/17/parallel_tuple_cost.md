---
name: "parallel_tuple_cost"
version: "17"
type: "floating point"
category: "Query Tuning / Planner Cost Constants"
short_desc: "Sets the planner's estimate of the cost of passing each tuple (row) from worker to leader backend."
context: "user"
default: "0.1"
min: "0"
max: "1.79769e+308"
url: "https://www.postgresql.org/docs/17/runtime-config-query.html#GUC-PARALLEL-TUPLE-COST"
---

Sets the planner's estimate of the cost of transferring one tuple from a parallel worker process to another process. The default is 0.1.
