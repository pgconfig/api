---
name: "enable_memoize"
version: "16"
type: "boolean"
category: "Query Tuning / Planner Method Configuration"
short_desc: "Enables the planner's use of memoization."
context: "user"
default: "on"
url: "https://www.postgresql.org/docs/16/runtime-config-query.html#GUC-ENABLE-MEMOIZE"
---

Enables or disables the query planner's use of memoize plans for caching results from parameterized scans inside nested-loop joins. This plan type allows scans to the underlying plans to be skipped when the results for the current parameters are already in the cache. Less commonly looked up results may be evicted from the cache when more space is required for new entries. The default is `on`.
