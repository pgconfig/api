---
name: "transaction_isolation"
version: "17"
type: "enum"
category: "Client Connection Defaults / Statement Behavior"
short_desc: "Sets the current transaction's isolation level."
context: "user"
default: "read committed"
values: ["serializable", "repeatable read", "read committed", "read uncommitted"]
url: "https://www.postgresql.org/docs/17/runtime-config-client.html#GUC-TRANSACTION-ISOLATION"
---

This parameter reflects the current transaction's isolation level. At the beginning of each transaction, it is set to the current value of [`default_transaction_isolation`](https://www.postgresql.org/docs/17/runtime-config-client.html#GUC-DEFAULT-TRANSACTION-ISOLATION). Any subsequent attempt to change it is equivalent to a [SET TRANSACTION](https://www.postgresql.org/docs/17/sql-set-transaction.html) command.
