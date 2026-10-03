---
name: "default_transaction_read_only"
version: "13"
type: "boolean"
category: "Client Connection Defaults / Statement Behavior"
short_desc: "Sets the default read-only status of new transactions."
context: "user"
default: "off"
url: "https://www.postgresql.org/docs/13/runtime-config-client.html#GUC-DEFAULT-TRANSACTION-READ-ONLY"
---

A read-only SQL transaction cannot alter non-temporary tables. This parameter controls the default read-only status of each new transaction. The default is `off` (read/write).

Consult [SET TRANSACTION](https://www.postgresql.org/docs/13/sql-set-transaction.html) for more information.
