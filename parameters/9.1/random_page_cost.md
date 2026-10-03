---
name: "random_page_cost"
version: "9.1"
type: "floating point"
category: "Query Tuning / Planner Cost Constants"
short_desc: "Sets the planner's estimate of the cost of a nonsequentially fetched disk page."
context: "user"
default: "4"
min: "0"
max: "1.79769e+308"
url: "https://www.postgresql.org/docs/9.1/runtime-config-query.html#GUC-RANDOM-PAGE-COST"
---

Sets the planner's estimate of the cost of a non-sequentially-fetched disk page. The default is 4.0. This value can be overridden for a particular tablespace by setting the tablespace parameter of the same name (see [ALTER TABLESPACE](https://www.postgresql.org/docs/9.1/sql-altertablespace.html)).

Reducing this value relative to `seq_page_cost` will cause the system to prefer index scans; raising it will make index scans look relatively more expensive. You can raise or lower both values together to change the importance of disk I/O costs relative to CPU costs, which are described by the following parameters.

> [!TIP]
> Although the system will let you set `random_page_cost` to less than `seq_page_cost`, it is not physically sensible to do so. However, setting them equal makes sense if the database is entirely cached in RAM, since in that case there is no penalty for touching pages out of sequence. Also, in a heavily-cached database you should lower both values relative to the CPU parameters, since the cost of fetching a page already in RAM is much smaller than it would normally be.
