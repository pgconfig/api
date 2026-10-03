---
name: "log_autovacuum_min_duration"
version: "9.4"
type: "integer"
category: "Reporting and Logging / What to Log"
short_desc: "Sets the minimum execution time above which autovacuum actions will be logged."
extra_desc: "Zero prints all actions. -1 turns autovacuum logging off."
context: "sighup"
unit: "ms"
default: "-1"
min: "-1"
max: "2147483647"
url: "https://www.postgresql.org/docs/9.4/runtime-config-autovacuum.html#GUC-LOG-AUTOVACUUM-MIN-DURATION"
---

Causes each action executed by autovacuum to be logged if it ran for at least the specified number of milliseconds. Setting this to zero logs all autovacuum actions. Minus-one (the default) disables logging autovacuum actions. For example, if you set this to `250ms` then all automatic vacuums and analyzes that run 250ms or longer will be logged. In addition, when this parameter is set to any value other than `-1`, a message will be logged if an autovacuum action is skipped due to the existence of a conflicting lock. Enabling this parameter can be helpful in tracking autovacuum activity. This setting can only be set in the `postgresql.conf` file or on the server command line.
