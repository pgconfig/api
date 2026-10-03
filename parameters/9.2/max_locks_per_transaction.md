---
name: "max_locks_per_transaction"
version: "9.2"
type: "integer"
category: "Lock Management"
short_desc: "Sets the maximum number of locks per transaction."
extra_desc: "The shared lock table is sized on the assumption that at most max_locks_per_transaction * max_connections distinct objects will need to be locked at any one time."
context: "postmaster"
default: "64"
min: "10"
max: "2147483647"
url: "https://www.postgresql.org/docs/9.2/runtime-config-locks.html#GUC-MAX-LOCKS-PER-TRANSACTION"
---

The shared lock table tracks locks on `max_locks_per_transaction` \* ([`max_connections`](https://www.postgresql.org/docs/9.2/runtime-config-connection.html#GUC-MAX-CONNECTIONS) + [`max_prepared_transactions`](https://www.postgresql.org/docs/9.2/runtime-config-resource.html#GUC-MAX-PREPARED-TRANSACTIONS)) objects (e.g., tables); hence, no more than this many distinct objects can be locked at any one time. This parameter controls the average number of object locks allocated for each transaction; individual transactions can lock more objects as long as the locks of all transactions fit in the lock table. This is *not* the number of rows that can be locked; that value is unlimited. The default, 64, has historically proven sufficient, but you might need to raise this value if you have clients that touch many different tables in a single transaction. This parameter can only be set at server start.

Increasing this parameter might cause PostgreSQL to request more `System V` shared memory than your operating system's default configuration allows. See [Shared Memory and Semaphores](https://www.postgresql.org/docs/9.2/kernel-resources.html#SYSVIPC) for information on how to adjust those parameters, if necessary.

When running a standby server, you must set this parameter to the same or higher value than on the master server. Otherwise, queries will not be allowed in the standby server.
