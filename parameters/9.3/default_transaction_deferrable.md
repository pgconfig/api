---
name: "default_transaction_deferrable"
version: "9.3"
type: "boolean"
category: "Client Connection Defaults / Statement Behavior"
short_desc: "Sets the default deferrable status of new transactions."
context: "user"
default: "off"
url: "https://www.postgresql.org/docs/9.3/runtime-config-client.html#GUC-DEFAULT-TRANSACTION-DEFERRABLE"
---

When running at the `serializable` isolation level, a deferrable read-only SQL transaction may be delayed before it is allowed to proceed. However, once it begins executing it does not incur any of the overhead required to ensure serializability; so serialization code will have no reason to force it to abort because of concurrent updates, making this option suitable for long-running read-only transactions.

This parameter controls the default deferrable status of each new transaction. It currently has no effect on read-write transactions or those operating at isolation levels lower than `serializable`. The default is `off`.

Consult [SET TRANSACTION](https://www.postgresql.org/docs/9.3/sql-set-transaction.html) for more information.
