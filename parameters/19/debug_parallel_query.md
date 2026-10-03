---
name: "debug_parallel_query"
version: "19"
type: "enum"
category: "Developer Options"
short_desc: "Forces the planner's use parallel query nodes."
extra_desc: "This can be useful for testing the parallel query infrastructure by forcing the planner to generate plans that contain nodes that perform tuple communication between workers and the main process."
context: "user"
default: "off"
values: ["off", "on", "regress"]
url: "https://www.postgresql.org/docs/19/runtime-config-developer.html#GUC-DEBUG-PARALLEL-QUERY"
---

Allows the use of parallel queries for testing purposes even in cases where no performance benefit is expected. The allowed values of `debug_parallel_query` are `off` (use parallel mode only when it is expected to improve performance), `on` (force parallel query for all queries for which it is thought to be safe), and `regress` (like `on`, but with additional behavior changes as explained below).

More specifically, setting this value to `on` will add a `Gather` node to the top of any query plan for which this appears to be safe, so that the query runs inside of a parallel worker. Even when a parallel worker is not available or cannot be used, operations such as starting a subtransaction that would be prohibited in a parallel query context will be prohibited unless the planner believes that this will cause the query to fail. If failures or unexpected results occur when this option is set, some functions used by the query may need to be marked `PARALLEL UNSAFE` (or, possibly, `PARALLEL RESTRICTED`).

Setting this value to `regress` has all of the same effects as setting it to `on` plus some additional effects that are intended to facilitate automated regression testing. Normally, messages from a parallel worker include a context line indicating that, but a setting of `regress` suppresses this line so that the output is the same as in non-parallel execution. Also, the `Gather` nodes added to plans by this setting are hidden in `EXPLAIN` output so that the output matches what would be obtained if this setting were turned `off`.
