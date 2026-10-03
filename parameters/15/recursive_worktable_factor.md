---
name: "recursive_worktable_factor"
version: "15"
type: "floating point"
category: "Query Tuning / Other Planner Options"
short_desc: "Sets the planner's estimate of the average size of a recursive query's working table."
context: "user"
default: "10"
min: "0.001"
max: "1e+06"
url: "https://www.postgresql.org/docs/15/runtime-config-query.html#GUC-RECURSIVE-WORKTABLE-FACTOR"
---

Sets the planner's estimate of the average size of the working table of a [recursive query](https://www.postgresql.org/docs/15/queries-with.html#QUERIES-WITH-RECURSIVE), as a multiple of the estimated size of the initial non-recursive term of the query. This helps the planner choose the most appropriate method for joining the working table to the query's other tables. The default value is `10.0`. A smaller value such as `1.0` can be helpful when the recursion has low "fan-out" from one step to the next, as for example in shortest-path queries. Graph analytics queries may benefit from larger-than-default values.
