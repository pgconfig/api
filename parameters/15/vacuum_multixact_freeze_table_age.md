---
name: "vacuum_multixact_freeze_table_age"
version: "15"
type: "integer"
category: "Client Connection Defaults / Statement Behavior"
short_desc: "Multixact age at which VACUUM should scan whole table to freeze tuples."
context: "user"
default: "150000000"
min: "0"
max: "2000000000"
url: "https://www.postgresql.org/docs/15/runtime-config-client.html#GUC-VACUUM-MULTIXACT-FREEZE-TABLE-AGE"
---

`VACUUM` performs an aggressive scan if the table's `pg_class`.`relminmxid` field has reached the age specified by this setting. An aggressive scan differs from a regular `VACUUM` in that it visits every page that might contain unfrozen XIDs or MXIDs, not just those that might contain dead tuples. The default is 150 million multixacts. Although users can set this value anywhere from zero to two billion, `VACUUM` will silently limit the effective value to 95% of [`autovacuum_multixact_freeze_max_age`](https://www.postgresql.org/docs/15/runtime-config-autovacuum.html#GUC-AUTOVACUUM-MULTIXACT-FREEZE-MAX-AGE), so that a periodic manual `VACUUM` has a chance to run before an anti-wraparound is launched for the table. For more information see [Multixacts and Wraparound](https://www.postgresql.org/docs/15/routine-vacuuming.html#VACUUM-FOR-MULTIXACT-WRAPAROUND).
