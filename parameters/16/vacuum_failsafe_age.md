---
name: "vacuum_failsafe_age"
version: "16"
type: "integer"
category: "Client Connection Defaults / Statement Behavior"
short_desc: "Age at which VACUUM should trigger failsafe to avoid a wraparound outage."
context: "user"
default: "1600000000"
min: "0"
max: "2100000000"
url: "https://www.postgresql.org/docs/16/runtime-config-client.html#GUC-VACUUM-FAILSAFE-AGE"
---

Specifies the maximum age (in transactions) that a table's `pg_class`.`relfrozenxid` field can attain before `VACUUM` takes extraordinary measures to avoid system-wide transaction ID wraparound failure. This is `VACUUM`'s strategy of last resort. The failsafe typically triggers when an autovacuum to prevent transaction ID wraparound has already been running for some time, though it's possible for the failsafe to trigger during any `VACUUM`.

When the failsafe is triggered, any cost-based delay that is in effect will no longer be applied, further non-essential maintenance tasks (such as index vacuuming) are bypassed, and any [Buffer Access Strategy](https://www.postgresql.org/docs/16/glossary.html#GLOSSARY-BUFFER-ACCESS-STRATEGY) in use will be disabled resulting in `VACUUM` being free to make use of all of [shared buffers](https://www.postgresql.org/docs/16/glossary.html#GLOSSARY-SHARED-MEMORY).

The default is 1.6 billion transactions. Although users can set this value anywhere from zero to 2.1 billion, `VACUUM` will silently adjust the effective value to no less than 105% of [`autovacuum_freeze_max_age`](https://www.postgresql.org/docs/16/runtime-config-autovacuum.html#GUC-AUTOVACUUM-FREEZE-MAX-AGE).
