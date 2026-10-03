---
name: "geqo_effort"
version: "19"
type: "integer"
category: "Query Tuning / Genetic Query Optimizer"
short_desc: "GEQO: effort is used to set the default for other GEQO parameters."
context: "user"
default: "5"
min: "1"
max: "10"
url: "https://www.postgresql.org/docs/19/runtime-config-query.html#GUC-GEQO-EFFORT"
---

Controls the trade-off between planning time and query plan quality in GEQO. This variable must be an integer in the range from 1 to 10. The default value is five. Larger values increase the time spent doing query planning, but also increase the likelihood that an efficient query plan will be chosen.

`geqo_effort` doesn't actually do anything directly; it is only used to compute the default values for the other variables that influence GEQO behavior (described below). If you prefer, you can set the other parameters by hand instead.
