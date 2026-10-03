---
name: "vacuum_freeze_table_age"
version: "9.3"
type: "integer"
category: "Client Connection Defaults / Statement Behavior"
short_desc: "Age at which VACUUM should scan whole table to freeze tuples."
context: "user"
default: "150000000"
min: "0"
max: "2000000000"
url: "https://www.postgresql.org/docs/9.3/runtime-config-client.html#GUC-VACUUM-FREEZE-TABLE-AGE"
---

`VACUUM` performs a whole-table scan if the table's `pg_class`.`relfrozenxid` field has reached the age specified by this setting. The default is 150 million transactions. Although users can set this value anywhere from zero to two billions, `VACUUM` will silently limit the effective value to 95% of [`autovacuum_freeze_max_age`](https://www.postgresql.org/docs/9.3/runtime-config-autovacuum.html#GUC-AUTOVACUUM-FREEZE-MAX-AGE), so that a periodical manual `VACUUM` has a chance to run before an anti-wraparound autovacuum is launched for the table. For more information see [Preventing Transaction ID Wraparound Failures](https://www.postgresql.org/docs/9.3/routine-vacuuming.html#VACUUM-FOR-WRAPAROUND).
