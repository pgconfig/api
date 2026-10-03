---
name: "max_pred_locks_per_transaction"
version: "19"
type: "integer"
category: "Lock Management"
short_desc: "Sets the maximum number of predicate locks per transaction."
extra_desc: "The shared predicate lock table is sized on the assumption that at most \"max_pred_locks_per_transaction\" objects per server process or prepared transaction will need to be locked at any one time."
context: "postmaster"
default: "64"
min: "10"
max: "2147483647"
url: "https://www.postgresql.org/docs/19/runtime-config-locks.html#GUC-MAX-PRED-LOCKS-PER-TRANSACTION"
---

The shared predicate lock table has space for `max_pred_locks_per_transaction` objects (e.g., tables) per server process or prepared transaction; hence, no more than this many distinct objects can be locked at any one time. This parameter limits the average number of object locks used by each transaction; individual transactions can lock more objects as long as the locks of all transactions fit in the lock table. This is *not* the number of rows that can be locked; that value is unlimited. The default, 64, has historically proven sufficient, but you might need to raise this value if you have clients that touch many different tables in a single serializable transaction. This parameter can only be set at server start.
