---
name: "track_activities"
version: "9.4"
type: "boolean"
category: "Statistics / Query and Index Statistics Collector"
short_desc: "Collects information about executing commands."
extra_desc: "Enables the collection of information on the currently executing command of each session, along with the time at which that command began execution."
context: "superuser"
default: "on"
url: "https://www.postgresql.org/docs/9.4/runtime-config-statistics.html#GUC-TRACK-ACTIVITIES"
---

Enables the collection of information on the currently executing command of each session, along with the time when that command began execution. This parameter is on by default. Note that even when enabled, this information is not visible to all users, only to superusers and the user owning the session being reported on, so it should not represent a security risk. Only superusers can change this setting.
