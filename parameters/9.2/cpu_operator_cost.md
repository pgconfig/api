---
name: "cpu_operator_cost"
version: "9.2"
type: "floating point"
category: "Query Tuning / Planner Cost Constants"
short_desc: "Sets the planner's estimate of the cost of processing each operator or function call."
context: "user"
default: "0.0025"
min: "0"
max: "1.79769e+308"
url: "https://www.postgresql.org/docs/9.2/runtime-config-query.html#GUC-CPU-OPERATOR-COST"
---

Sets the planner's estimate of the cost of processing each operator or function executed during a query. The default is 0.0025.
