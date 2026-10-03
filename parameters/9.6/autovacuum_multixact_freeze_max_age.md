---
name: "autovacuum_multixact_freeze_max_age"
version: "9.6"
type: "integer"
category: "Autovacuum"
short_desc: "Multixact age at which to autovacuum a table to prevent multixact wraparound."
context: "postmaster"
default: "400000000"
min: "10000"
max: "2000000000"
url: "https://www.postgresql.org/docs/9.6/runtime-config-autovacuum.html#GUC-AUTOVACUUM-MULTIXACT-FREEZE-MAX-AGE"
---

Specifies the maximum age (in multixacts) that a table's `pg_class`.`relminmxid` field can attain before a `VACUUM` operation is forced to prevent multixact ID wraparound within the table. Note that the system will launch autovacuum processes to prevent wraparound even when autovacuum is otherwise disabled.

Vacuuming multixacts also allows removal of old files from the `pg_multixact/members` and `pg_multixact/offsets` subdirectories, which is why the default is a relatively low 400 million multixacts. This parameter can only be set at server start, but the setting can be reduced for individual tables by changing table storage parameters. For more information see [Multixacts and Wraparound](https://www.postgresql.org/docs/9.6/routine-vacuuming.html#VACUUM-FOR-MULTIXACT-WRAPAROUND).
