---
name: "default_transaction_isolation"
version: "17"
type: "enum"
category: "Client Connection Defaults / Statement Behavior"
short_desc: "Sets the transaction isolation level of each new transaction."
context: "user"
default: "read committed"
values: ["serializable", "repeatable read", "read committed", "read uncommitted"]
url: "https://www.postgresql.org/docs/17/runtime-config-client.html#GUC-DEFAULT-TRANSACTION-ISOLATION"
---

Each SQL transaction has an isolation level, which can be either "read uncommitted", "read committed", "repeatable read", or "serializable". This parameter controls the default isolation level of each new transaction. The default is "read committed".

Consult [Concurrency Control](https://www.postgresql.org/docs/17/mvcc.html) and [SET TRANSACTION](https://www.postgresql.org/docs/17/sql-set-transaction.html) for more information.
