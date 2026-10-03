---
name: "join_collapse_limit"
version: "18"
type: "integer"
category: "Query Tuning / Other Planner Options"
short_desc: "Sets the FROM-list size beyond which JOIN constructs are not flattened."
extra_desc: "The planner will flatten explicit JOIN constructs into lists of FROM items whenever a list of no more than this many items would result."
context: "user"
default: "8"
min: "1"
max: "2147483647"
url: "https://www.postgresql.org/docs/18/runtime-config-query.html#GUC-JOIN-COLLAPSE-LIMIT"
---

The planner will rewrite explicit `JOIN` constructs (except `FULL JOIN`s) into lists of `FROM` items whenever a list of no more than this many items would result. Smaller values reduce planning time but might yield inferior query plans.

By default, this variable is set the same as `from_collapse_limit`, which is appropriate for most uses. Setting it to 1 prevents any reordering of explicit `JOIN`s. Thus, the explicit join order specified in the query will be the actual order in which the relations are joined. Because the query planner does not always choose the optimal join order, advanced users can elect to temporarily set this variable to 1, and then specify the join order they desire explicitly. For more information see [Controlling the Planner with Explicit JOIN Clauses](https://www.postgresql.org/docs/18/explicit-joins.html).

Setting this value to [`geqo_threshold`](https://www.postgresql.org/docs/18/runtime-config-query.html#GUC-GEQO-THRESHOLD) or more may trigger use of the GEQO planner, resulting in non-optimal plans. See [Genetic Query Optimizer](https://www.postgresql.org/docs/18/runtime-config-query.html#RUNTIME-CONFIG-QUERY-GEQO).
