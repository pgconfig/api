---
name: "geqo"
version: "9.3"
type: "boolean"
category: "Query Tuning / Genetic Query Optimizer"
short_desc: "Enables genetic query optimization."
extra_desc: "This algorithm attempts to do planning without exhaustive searching."
context: "user"
default: "on"
url: "https://www.postgresql.org/docs/9.3/runtime-config-query.html#GUC-GEQO"
---

Enables or disables genetic query optimization. This is on by default. It is usually best not to turn it off in production; the `geqo_threshold` variable provides more granular control of GEQO.
