---
name: "vacuum_max_eager_freeze_failure_rate"
version: "19"
type: "floating point"
category: "Vacuuming / Freezing"
short_desc: "Fraction of pages in a relation vacuum can scan and fail to freeze before disabling eager scanning."
extra_desc: "A value of 0.0 disables eager scanning and a value of 1.0 will eagerly scan up to 100 percent of the all-visible pages in the relation. If vacuum successfully freezes these pages, the cap is lower than 100 percent, because the goal is to amortize page freezing across multiple vacuums."
context: "user"
default: "0.03"
min: "0"
max: "1"
url: "https://www.postgresql.org/docs/19/runtime-config-vacuum.html#GUC-VACUUM-MAX-EAGER-FREEZE-FAILURE-RATE"
---

Specifies the maximum number of pages (as a fraction of total pages in the relation) that `VACUUM` may scan and *fail* to set all-frozen in the visibility map before disabling eager scanning. A value of `0` disables eager scanning altogether. The default is `0.03` (3%).

Note that when eager scanning is enabled, only freeze failures count against the cap, not successful freezing. Successful page freezes are capped internally at 20% of the all-visible but not all-frozen pages in the relation. Capping successful page freezes helps amortize the overhead across multiple normal vacuums and limits the potential downside of wasted eager freezes of pages that are modified again before the next aggressive vacuum.

This parameter can only be set in the `postgresql.conf` file or on the server command line; but the setting can be overridden for individual tables by changing the [corresponding table storage parameter](https://www.postgresql.org/docs/19/sql-createtable.html#RELOPTION-VACUUM-MAX-EAGER-FREEZE-FAILURE-RATE). For more information on tuning vacuum's freezing behavior, see [Preventing Transaction ID Wraparound Failures](https://www.postgresql.org/docs/19/routine-vacuuming.html#VACUUM-FOR-WRAPAROUND).
