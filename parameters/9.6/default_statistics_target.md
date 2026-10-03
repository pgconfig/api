---
name: "default_statistics_target"
version: "9.6"
type: "integer"
category: "Query Tuning / Other Planner Options"
short_desc: "Sets the default statistics target."
extra_desc: "This applies to table columns that have not had a column-specific target set via ALTER TABLE SET STATISTICS."
context: "user"
default: "100"
min: "1"
max: "10000"
url: "https://www.postgresql.org/docs/9.6/runtime-config-query.html#GUC-DEFAULT-STATISTICS-TARGET"
---

Sets the default statistics target for table columns without a column-specific target set via `ALTER TABLE SET STATISTICS`. Larger values increase the time needed to do `ANALYZE`, but might improve the quality of the planner's estimates. The default is 100. For more information on the use of statistics by the PostgreSQL query planner, refer to [Statistics Used by the Planner](https://www.postgresql.org/docs/9.6/planner-stats.html).
