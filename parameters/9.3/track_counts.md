---
name: "track_counts"
version: "9.3"
type: "boolean"
category: "Statistics / Query and Index Statistics Collector"
short_desc: "Collects statistics on database activity."
context: "superuser"
default: "on"
url: "https://www.postgresql.org/docs/9.3/runtime-config-statistics.html#GUC-TRACK-COUNTS"
---

Enables collection of statistics on database activity. This parameter is on by default, because the autovacuum daemon needs the collected information. Only superusers can change this setting.
