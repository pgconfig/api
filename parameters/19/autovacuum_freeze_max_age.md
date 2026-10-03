---
name: "autovacuum_freeze_max_age"
version: "19"
type: "integer"
category: "Vacuuming / Automatic Vacuuming"
short_desc: "Age at which to autovacuum a table to prevent transaction ID wraparound."
context: "postmaster"
default: "200000000"
min: "100000"
max: "2000000000"
url: "https://www.postgresql.org/docs/19/runtime-config-vacuum.html#GUC-AUTOVACUUM-FREEZE-MAX-AGE"
---

Specifies the maximum age (in transactions) that a table's `pg_class`.`relfrozenxid` field can attain before a `VACUUM` operation is forced to prevent transaction ID wraparound within the table. Note that the system will launch autovacuum processes to prevent wraparound even when autovacuum is otherwise disabled.

Vacuum also allows removal of old files from the `pg_xact` subdirectory, which is why the default is a relatively low 200 million transactions. This parameter can only be set at server start, but the setting can be reduced for individual tables by changing table storage parameters. For more information see [Preventing Transaction ID Wraparound Failures](https://www.postgresql.org/docs/19/routine-vacuuming.html#VACUUM-FOR-WRAPAROUND).
