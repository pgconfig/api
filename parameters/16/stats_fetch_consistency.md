---
name: "stats_fetch_consistency"
version: "16"
type: "enum"
category: "Statistics / Cumulative Query and Index Statistics"
short_desc: "Sets the consistency of accesses to statistics data."
context: "user"
default: "cache"
values: ["none", "cache", "snapshot"]
url: "https://www.postgresql.org/docs/16/runtime-config-statistics.html#GUC-STATS-FETCH-CONSISTENCY"
---

Determines the behavior when cumulative statistics are accessed multiple times within a transaction. When set to `none`, each access re-fetches counters from shared memory. When set to `cache`, the first access to statistics for an object caches those statistics until the end of the transaction unless `pg_stat_clear_snapshot()` is called. When set to `snapshot`, the first statistics access caches all statistics accessible in the current database, until the end of the transaction unless `pg_stat_clear_snapshot()` is called. Changing this parameter in a transaction discards the statistics snapshot. The default is `cache`.

> [!NOTE]
> `none` is most suitable for monitoring systems. If values are only accessed once, it is the most efficient. `cache` ensures repeat accesses yield the same values, which is important for queries involving e.g. self-joins. `snapshot` can be useful when interactively inspecting statistics, but has higher overhead, particularly if many database objects exist.
