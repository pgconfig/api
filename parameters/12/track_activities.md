---
name: "track_activities"
version: "12"
type: "boolean"
category: "Statistics / Query and Index Statistics Collector"
short_desc: "Collects information about executing commands."
extra_desc: "Enables the collection of information on the currently executing command of each session, along with the time at which that command began execution."
context: "superuser"
default: "on"
url: "https://www.postgresql.org/docs/12/runtime-config-statistics.html#GUC-TRACK-ACTIVITIES"
---

Enables the collection of information on the currently executing command of each session, along with the time when that command began execution. This parameter is on by default. Note that even when enabled, this information is not visible to all users, only to superusers, roles with privileges of the `pg_read_all_stats` role and the user owning the sessions being reported on (including sessions belonging to a role they have the privileges of), so it should not represent a security risk. Only superusers can change this setting.
