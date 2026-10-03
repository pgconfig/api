---
name: "transaction_deferrable"
version: "10"
type: "boolean"
category: "Client Connection Defaults / Statement Behavior"
short_desc: "Whether to defer a read-only serializable transaction until it can be executed with no possible serialization failures."
context: "user"
default: "off"
url: "https://www.postgresql.org/docs/10/runtime-config-client.html#GUC-TRANSACTION-DEFERRABLE"
---

This parameter reflects the current transaction's deferrability status. At the beginning of each transaction, it is set to the current value of [`default_transaction_deferrable`](https://www.postgresql.org/docs/10/runtime-config-client.html#GUC-DEFAULT-TRANSACTION-DEFERRABLE). Any subsequent attempt to change it is equivalent to a [SET TRANSACTION](https://www.postgresql.org/docs/10/sql-set-transaction.html) command.
