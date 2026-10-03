---
name: "autovacuum_vacuum_score_weight"
version: "19"
type: "floating point"
category: "Vacuuming / Automatic Vacuuming"
short_desc: "Scaling factor of vacuum score for autovacuum prioritization."
context: "sighup"
default: "1"
min: "0"
max: "10"
url: "https://www.postgresql.org/docs/19/runtime-config-vacuum.html#GUC-AUTOVACUUM-VACUUM-SCORE-WEIGHT"
---

Specifies the scaling factor of the vacuum threshold component of the score used by autovacuum for prioritization purposes. The default is `1.0`. This parameter can only be set in the `postgresql.conf` file or on the server command line. See [Autovacuum Prioritization](https://www.postgresql.org/docs/19/routine-vacuuming.html#AUTOVACUUM-PRIORITY) for more information.
