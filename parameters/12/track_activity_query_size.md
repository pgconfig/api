---
name: "track_activity_query_size"
version: "12"
type: "integer"
category: "Resource Usage / Memory"
short_desc: "Sets the size reserved for pg_stat_activity.query, in bytes."
context: "postmaster"
unit: "B"
default: "1024"
min: "100"
max: "102400"
url: "https://www.postgresql.org/docs/12/runtime-config-statistics.html#GUC-TRACK-ACTIVITY-QUERY-SIZE"
---

Specifies the amount of memory reserved to store the text of the currently executing command for each active session, for the `pg_stat_activity`.`query` field. If this value is specified without units, it is taken as bytes. The default value is 1024 bytes. This parameter can only be set at server start.
