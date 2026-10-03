---
name: "enable_group_by_reordering"
version: "19"
type: "boolean"
category: "Query Tuning / Planner Method Configuration"
short_desc: "Enables reordering of GROUP BY keys."
context: "user"
default: "on"
url: "https://www.postgresql.org/docs/19/runtime-config-query.html#GUC-ENABLE-GROUP-BY-REORDERING"
---

Controls if the query planner will produce a plan which will provide `GROUP BY` keys sorted in the order of keys of a child node of the plan, such as an index scan. When disabled, the query planner will produce a plan with `GROUP BY` keys only sorted to match the `ORDER BY` clause, if any. When enabled, the planner will try to produce a more efficient plan. The default value is `on`.
