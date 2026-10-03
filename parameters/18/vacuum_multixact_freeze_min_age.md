---
name: "vacuum_multixact_freeze_min_age"
version: "18"
type: "integer"
category: "Vacuuming / Freezing"
short_desc: "Minimum age at which VACUUM should freeze a MultiXactId in a table row."
context: "user"
default: "5000000"
min: "0"
max: "1000000000"
url: "https://www.postgresql.org/docs/18/runtime-config-vacuum.html#GUC-VACUUM-MULTIXACT-FREEZE-MIN-AGE"
---

Specifies the cutoff age (in multixacts) that `VACUUM` should use to decide whether to trigger freezing of pages with an older multixact ID. The default is 5 million multixacts. Although users can set this value anywhere from zero to one billion, `VACUUM` will silently limit the effective value to half the value of [`autovacuum_multixact_freeze_max_age`](https://www.postgresql.org/docs/18/runtime-config-vacuum.html#GUC-AUTOVACUUM-MULTIXACT-FREEZE-MAX-AGE), so that there is not an unreasonably short time between forced autovacuums. For more information see [Multixacts and Wraparound](https://www.postgresql.org/docs/18/routine-vacuuming.html#VACUUM-FOR-MULTIXACT-WRAPAROUND).
