---
name: "from_collapse_limit"
version: "18"
type: "integer"
category: "Query Tuning / Other Planner Options"
short_desc: "Sets the FROM-list size beyond which subqueries are not collapsed."
extra_desc: "The planner will merge subqueries into upper queries if the resulting FROM list would have no more than this many items."
context: "user"
default: "8"
min: "1"
max: "2147483647"
url: "https://www.postgresql.org/docs/18/runtime-config-query.html#GUC-FROM-COLLAPSE-LIMIT"
---

The planner will merge sub-queries into upper queries if the resulting `FROM` list would have no more than this many items. Smaller values reduce planning time but might yield inferior query plans. The default is eight. For more information see [Controlling the Planner with Explicit JOIN Clauses](https://www.postgresql.org/docs/18/explicit-joins.html).

Setting this value to [`geqo_threshold`](https://www.postgresql.org/docs/18/runtime-config-query.html#GUC-GEQO-THRESHOLD) or more may trigger use of the GEQO planner, resulting in non-optimal plans. See [Genetic Query Optimizer](https://www.postgresql.org/docs/18/runtime-config-query.html#RUNTIME-CONFIG-QUERY-GEQO).
