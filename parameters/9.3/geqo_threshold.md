---
name: "geqo_threshold"
version: "9.3"
type: "integer"
category: "Query Tuning / Genetic Query Optimizer"
short_desc: "Sets the threshold of FROM items beyond which GEQO is used."
context: "user"
default: "12"
min: "2"
max: "2147483647"
url: "https://www.postgresql.org/docs/9.3/runtime-config-query.html#GUC-GEQO-THRESHOLD"
---

Use genetic query optimization to plan queries with at least this many `FROM` items involved. (Note that a `FULL OUTER JOIN` construct counts as only one `FROM` item.) The default is 12. For simpler queries it is usually best to use the regular, exhaustive-search planner, but for queries with many tables the exhaustive search takes too long, often longer than the penalty of executing a suboptimal plan. Thus, a threshold on the size of the query is a convenient way to manage use of GEQO.
