---
name: "track_counts"
version: "15"
type: "boolean"
category: "Statistics / Cumulative Query and Index Statistics"
short_desc: "Collects statistics on database activity."
context: "superuser"
default: "on"
url: "https://www.postgresql.org/docs/15/runtime-config-statistics.html#GUC-TRACK-COUNTS"
---

Enables collection of statistics on database activity. This parameter is on by default, because the autovacuum daemon needs the collected information. Only superusers and users with the appropriate `SET` privilege can change this setting.
