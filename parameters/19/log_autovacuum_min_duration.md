---
name: "log_autovacuum_min_duration"
version: "19"
type: "integer"
category: "Reporting and Logging / What to Log"
short_desc: "Sets the minimum execution time above which vacuum actions by autovacuum will be logged."
extra_desc: "-1 disables logging vacuum actions by autovacuum. 0 means log all vacuum actions by autovacuum."
context: "sighup"
unit: "ms"
default: "600000"
min: "-1"
max: "2147483647"
url: "https://www.postgresql.org/docs/19/runtime-config-logging.html#GUC-LOG-AUTOVACUUM-MIN-DURATION"
---

Causes vacuum action executed by autovacuum to be logged if it ran for at least the specified amount of time. Setting this to zero logs all vacuum actions by autovacuum. `-1` disables logging vacuum actions by autovacuum. If this value is specified without units, it is taken as milliseconds. For example, if you set this to `250ms` then all automatic vacuums that run 250ms or longer will be logged. In addition, when this parameter is set to any value other than `-1`, a message will be logged if a vacuum action by autovacuum is skipped due to a conflicting lock or a concurrently dropped relation. The default is `10min`. Enabling this parameter can be helpful in tracking vacuum activity by autovacuum. This parameter can only be set in the `postgresql.conf` file or on the server command line; but the setting can be overridden for individual tables by changing table storage parameters.
