---
name: "geqo_pool_size"
version: "15"
type: "integer"
category: "Query Tuning / Genetic Query Optimizer"
short_desc: "GEQO: number of individuals in the population."
extra_desc: "Zero selects a suitable default value."
context: "user"
default: "0"
min: "0"
max: "2147483647"
url: "https://www.postgresql.org/docs/15/runtime-config-query.html#GUC-GEQO-POOL-SIZE"
---

Controls the pool size used by GEQO, that is the number of individuals in the genetic population. It must be at least two, and useful values are typically 100 to 1000. If it is set to zero (the default setting) then a suitable value is chosen based on `geqo_effort` and the number of tables in the query.
