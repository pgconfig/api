---
name: "track_functions"
version: "15"
type: "enum"
category: "Statistics / Cumulative Query and Index Statistics"
short_desc: "Collects function-level statistics on database activity."
context: "superuser"
default: "none"
values: ["none", "pl", "all"]
url: "https://www.postgresql.org/docs/15/runtime-config-statistics.html#GUC-TRACK-FUNCTIONS"
---

Enables tracking of function call counts and time used. Specify `pl` to track only procedural-language functions, `all` to also track SQL and C language functions. The default is `none`, which disables function statistics tracking. Only superusers and users with the appropriate `SET` privilege can change this setting.

> [!NOTE]
> SQL-language functions that are simple enough to be "inlined" into the calling query will not be tracked, regardless of this setting.
