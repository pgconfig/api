---
name: "parallel_setup_cost"
version: "17"
type: "floating point"
category: "Query Tuning / Planner Cost Constants"
short_desc: "Sets the planner's estimate of the cost of starting up worker processes for parallel query."
context: "user"
default: "1000"
min: "0"
max: "1.79769e+308"
url: "https://www.postgresql.org/docs/17/runtime-config-query.html#GUC-PARALLEL-SETUP-COST"
---

Sets the planner's estimate of the cost of launching parallel worker processes. The default is 1000.
