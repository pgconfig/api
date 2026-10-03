---
name: "transaction_read_only"
version: "19"
type: "boolean"
category: "Client Connection Defaults / Statement Behavior"
short_desc: "Sets the current transaction's read-only status."
context: "user"
default: "off"
url: "https://www.postgresql.org/docs/19/runtime-config-client.html#GUC-TRANSACTION-READ-ONLY"
---

This parameter reflects the current transaction's read-only status. At the beginning of each transaction, it is set to the current value of [`default_transaction_read_only`](https://www.postgresql.org/docs/19/runtime-config-client.html#GUC-DEFAULT-TRANSACTION-READ-ONLY). Any subsequent attempt to change it is equivalent to a [SET TRANSACTION](https://www.postgresql.org/docs/19/sql-set-transaction.html) command.
