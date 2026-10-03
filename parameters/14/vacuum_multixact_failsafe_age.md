---
name: "vacuum_multixact_failsafe_age"
version: "14"
type: "integer"
category: "Client Connection Defaults / Statement Behavior"
short_desc: "Multixact age at which VACUUM should trigger failsafe to avoid a wraparound outage."
context: "user"
default: "1600000000"
min: "0"
max: "2100000000"
url: "https://www.postgresql.org/docs/14/runtime-config-client.html#GUC-VACUUM-MULTIXACT-FAILSAFE-AGE"
---

Specifies the maximum age (in multixacts) that a table's `pg_class`.`relminmxid` field can attain before `VACUUM` takes extraordinary measures to avoid system-wide multixact ID wraparound failure. This is `VACUUM`'s strategy of last resort. The failsafe typically triggers when an autovacuum to prevent transaction ID wraparound has already been running for some time, though it's possible for the failsafe to trigger during any `VACUUM`.

When the failsafe is triggered, any cost-based delay that is in effect will no longer be applied, and further non-essential maintenance tasks (such as index vacuuming) are bypassed.

The default is 1.6 billion multixacts. Although users can set this value anywhere from zero to 2.1 billion, `VACUUM` will silently adjust the effective value to no less than 105% of [`autovacuum_multixact_freeze_max_age`](https://www.postgresql.org/docs/14/runtime-config-autovacuum.html#GUC-AUTOVACUUM-MULTIXACT-FREEZE-MAX-AGE).
