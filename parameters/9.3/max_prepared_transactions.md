---
name: "max_prepared_transactions"
version: "9.3"
type: "integer"
category: "Resource Usage / Memory"
short_desc: "Sets the maximum number of simultaneously prepared transactions."
context: "postmaster"
default: "0"
min: "0"
max: "8388607"
url: "https://www.postgresql.org/docs/9.3/runtime-config-resource.html#GUC-MAX-PREPARED-TRANSACTIONS"
---

Sets the maximum number of transactions that can be in the "prepared" state simultaneously (see [PREPARE TRANSACTION](https://www.postgresql.org/docs/9.3/sql-prepare-transaction.html)). Setting this parameter to zero (which is the default) disables the prepared-transaction feature. This parameter can only be set at server start.

If you are not planning to use prepared transactions, this parameter should be set to zero to prevent accidental creation of prepared transactions. If you are using prepared transactions, you will probably want `max_prepared_transactions` to be at least as large as [`max_connections`](https://www.postgresql.org/docs/9.3/runtime-config-connection.html#GUC-MAX-CONNECTIONS), so that every session can have a prepared transaction pending.

When running a standby server, you must set this parameter to the same or higher value than on the master server. Otherwise, queries will not be allowed in the standby server.
