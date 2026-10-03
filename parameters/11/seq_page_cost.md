---
name: "seq_page_cost"
version: "11"
type: "floating point"
category: "Query Tuning / Planner Cost Constants"
short_desc: "Sets the planner's estimate of the cost of a sequentially fetched disk page."
context: "user"
default: "1"
min: "0"
max: "1.79769e+308"
url: "https://www.postgresql.org/docs/11/runtime-config-query.html#GUC-SEQ-PAGE-COST"
---

Sets the planner's estimate of the cost of a disk page fetch that is part of a series of sequential fetches. The default is 1.0. This value can be overridden for tables and indexes in a particular tablespace by setting the tablespace parameter of the same name (see [ALTER TABLESPACE](https://www.postgresql.org/docs/11/sql-altertablespace.html)).
