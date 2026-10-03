---
name: "geqo_seed"
version: "9.3"
type: "floating point"
category: "Query Tuning / Genetic Query Optimizer"
short_desc: "GEQO: seed for random path selection."
context: "user"
default: "0"
min: "0"
max: "1"
url: "https://www.postgresql.org/docs/9.3/runtime-config-query.html#GUC-GEQO-SEED"
---

Controls the initial value of the random number generator used by GEQO to select random paths through the join order search space. The value can range from zero (the default) to one. Varying the value changes the set of join paths explored, and may result in a better or worse best path being found.
