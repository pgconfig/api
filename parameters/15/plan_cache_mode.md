---
name: "plan_cache_mode"
version: "15"
type: "enum"
category: "Query Tuning / Other Planner Options"
short_desc: "Controls the planner's selection of custom or generic plan."
extra_desc: "Prepared statements can have custom and generic plans, and the planner will attempt to choose which is better.  This can be set to override the default behavior."
context: "user"
default: "auto"
values: ["auto", "force_generic_plan", "force_custom_plan"]
url: "https://www.postgresql.org/docs/15/runtime-config-query.html#GUC-PLAN-CACHE_MODE"
---

Prepared statements (either explicitly prepared or implicitly generated, for example by PL/pgSQL) can be executed using custom or generic plans. Custom plans are made afresh for each execution using its specific set of parameter values, while generic plans do not rely on the parameter values and can be re-used across executions. Thus, use of a generic plan saves planning time, but if the ideal plan depends strongly on the parameter values then a generic plan may be inefficient. The choice between these options is normally made automatically, but it can be overridden with `plan_cache_mode`. The allowed values are `auto` (the default), `force_custom_plan` and `force_generic_plan`. This setting is considered when a cached plan is to be executed, not when it is prepared. For more information see [PREPARE](https://www.postgresql.org/docs/15/sql-prepare.html).
