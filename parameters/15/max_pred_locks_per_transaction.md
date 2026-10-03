---
name: "max_pred_locks_per_transaction"
version: "15"
type: "integer"
category: "Lock Management"
short_desc: "Sets the maximum number of predicate locks per transaction."
extra_desc: "The shared predicate lock table is sized on the assumption that at most max_pred_locks_per_transaction * max_connections distinct objects will need to be locked at any one time."
context: "postmaster"
default: "64"
min: "10"
max: "2147483647"
url: "https://www.postgresql.org/docs/15/runtime-config-locks.html#GUC-MAX-PRED-LOCKS-PER-TRANSACTION"
---

The shared predicate lock table tracks locks on `max_pred_locks_per_transaction` \* ([`max_connections`](https://www.postgresql.org/docs/15/runtime-config-connection.html#GUC-MAX-CONNECTIONS) + [`max_prepared_transactions`](https://www.postgresql.org/docs/15/runtime-config-resource.html#GUC-MAX-PREPARED-TRANSACTIONS)) objects (e.g., tables); hence, no more than this many distinct objects can be locked at any one time. This parameter controls the average number of object locks allocated for each transaction; individual transactions can lock more objects as long as the locks of all transactions fit in the lock table. This is *not* the number of rows that can be locked; that value is unlimited. The default, 64, has generally been sufficient in testing, but you might need to raise this value if you have clients that touch many different tables in a single serializable transaction. This parameter can only be set at server start.
