---
name: "track_activity_query_size"
version: "9.4"
type: "integer"
category: "Resource Usage / Memory"
short_desc: "Sets the size reserved for pg_stat_activity.query, in bytes."
context: "postmaster"
default: "1024"
min: "100"
max: "102400"
url: "https://www.postgresql.org/docs/9.4/runtime-config-statistics.html#GUC-TRACK-ACTIVITY-QUERY-SIZE"
---

Specifies the number of bytes reserved to track the currently executing command for each active session, for the `pg_stat_activity`.`query` field. The default value is 1024. This parameter can only be set at server start.
