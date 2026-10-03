---
name: "vacuum_freeze_min_age"
version: "18"
type: "integer"
category: "Vacuuming / Freezing"
short_desc: "Minimum age at which VACUUM should freeze a table row."
context: "user"
default: "50000000"
min: "0"
max: "1000000000"
url: "https://www.postgresql.org/docs/18/runtime-config-vacuum.html#GUC-VACUUM-FREEZE-MIN-AGE"
---

Specifies the cutoff age (in transactions) that `VACUUM` should use to decide whether to trigger freezing of pages that have an older XID. The default is 50 million transactions. Although users can set this value anywhere from zero to one billion, `VACUUM` will silently limit the effective value to half the value of [`autovacuum_freeze_max_age`](https://www.postgresql.org/docs/18/runtime-config-vacuum.html#GUC-AUTOVACUUM-FREEZE-MAX-AGE), so that there is not an unreasonably short time between forced autovacuums. For more information see [Preventing Transaction ID Wraparound Failures](https://www.postgresql.org/docs/18/routine-vacuuming.html#VACUUM-FOR-WRAPAROUND).
