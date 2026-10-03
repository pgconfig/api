---
name: "min_parallel_relation_size"
version: "9.6"
type: "integer"
category: "Query Tuning / Planner Cost Constants"
short_desc: "Sets the minimum size of relations to be considered for parallel scan."
context: "user"
unit: "8kB"
default: "1024"
min: "0"
max: "715827882"
url: "https://www.postgresql.org/docs/9.6/runtime-config-query.html#GUC-MIN-PARALLEL-RELATION-SIZE"
---

Sets the minimum size of relations to be considered for parallel scan. The default is 8 megabytes (`8MB`).
