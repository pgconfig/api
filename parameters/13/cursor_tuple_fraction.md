---
name: "cursor_tuple_fraction"
version: "13"
type: "floating point"
category: "Query Tuning / Other Planner Options"
short_desc: "Sets the planner's estimate of the fraction of a cursor's rows that will be retrieved."
context: "user"
default: "0.1"
min: "0"
max: "1"
url: "https://www.postgresql.org/docs/13/runtime-config-query.html#GUC-CURSOR-TUPLE-FRACTION"
---

Sets the planner's estimate of the fraction of a cursor's rows that will be retrieved. The default is 0.1. Smaller values of this setting bias the planner towards using "fast start" plans for cursors, which will retrieve the first few rows quickly while perhaps taking a long time to fetch all rows. Larger values put more emphasis on the total estimated time. At the maximum setting of 1.0, cursors are planned exactly like regular queries, considering only the total estimated time and not how soon the first rows might be delivered.
