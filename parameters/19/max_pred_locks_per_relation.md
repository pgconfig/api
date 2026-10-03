---
name: "max_pred_locks_per_relation"
version: "19"
type: "integer"
category: "Lock Management"
short_desc: "Sets the maximum number of predicate-locked pages and tuples per relation."
extra_desc: "If more than this total of pages and tuples in the same relation are locked by a connection, those locks are replaced by a relation-level lock."
context: "sighup"
default: "-2"
min: "-2147483648"
max: "2147483647"
url: "https://www.postgresql.org/docs/19/runtime-config-locks.html#GUC-MAX-PRED-LOCKS-PER-RELATION"
---

This controls how many pages or tuples of a single relation can be predicate-locked before the lock is promoted to covering the whole relation. Values greater than or equal to zero mean an absolute limit, while negative values mean [`max_pred_locks_per_transaction`](https://www.postgresql.org/docs/19/runtime-config-locks.html#GUC-MAX-PRED-LOCKS-PER-TRANSACTION) divided by the absolute value of this setting. The default is -2, which keeps the behavior from previous versions of PostgreSQL. This parameter can only be set in the `postgresql.conf` file or on the server command line.
