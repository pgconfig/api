---
name: "geqo_selection_bias"
version: "15"
type: "floating point"
category: "Query Tuning / Genetic Query Optimizer"
short_desc: "GEQO: selective pressure within the population."
context: "user"
default: "2"
min: "1.5"
max: "2"
url: "https://www.postgresql.org/docs/15/runtime-config-query.html#GUC-GEQO-SELECTION-BIAS"
---

Controls the selection bias used by GEQO. The selection bias is the selective pressure within the population. Values can be from 1.50 to 2.00; the latter is the default.
