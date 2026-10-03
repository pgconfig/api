---
name: "geqo_generations"
version: "15"
type: "integer"
category: "Query Tuning / Genetic Query Optimizer"
short_desc: "GEQO: number of iterations of the algorithm."
extra_desc: "Zero selects a suitable default value."
context: "user"
default: "0"
min: "0"
max: "2147483647"
url: "https://www.postgresql.org/docs/15/runtime-config-query.html#GUC-GEQO-GENERATIONS"
---

Controls the number of generations used by GEQO, that is the number of iterations of the algorithm. It must be at least one, and useful values are in the same range as the pool size. If it is set to zero (the default setting) then a suitable value is chosen based on `geqo_pool_size`.
