---
name: "vacuum_truncate"
version: "19"
type: "boolean"
category: "Vacuuming / Default Behavior"
short_desc: "Enables vacuum to truncate empty pages at the end of the table."
context: "user"
default: "on"
url: "https://www.postgresql.org/docs/19/runtime-config-vacuum.html#GUC-VACUUM-TRUNCATE"
---

Enables or disables vacuum to try to truncate off any empty pages at the end of the table. The default value is `true`. If `true`, `VACUUM` and autovacuum do the truncation and the disk space for the truncated pages is returned to the operating system. Note that the truncation requires an `ACCESS EXCLUSIVE` lock on the table. The `TRUNCATE` parameter of [`VACUUM`](https://www.postgresql.org/docs/19/sql-vacuum.html), if specified, overrides the value of this parameter. The setting can also be overridden for individual tables by changing table storage parameters.
