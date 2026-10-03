---
name: "min_eager_agg_group_size"
version: "19"
type: "floating point"
category: "Query Tuning / Planner Cost Constants"
short_desc: "Sets the minimum average group size required to consider applying eager aggregation."
context: "user"
default: "8"
min: "0"
max: "1.79769e+308"
url: "https://www.postgresql.org/docs/19/runtime-config-query.html#GUC-MIN-EAGER-AGG-GROUP-SIZE"
---

Sets the minimum average group size required to consider applying eager aggregation. This helps avoid the overhead of eager aggregation when it does not offer significant row count reduction. The default is `8`.
