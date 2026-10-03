---
name: "idle_in_transaction_session_timeout"
version: "17"
type: "integer"
category: "Client Connection Defaults / Statement Behavior"
short_desc: "Sets the maximum allowed idle time between queries, when in a transaction."
extra_desc: "A value of 0 turns off the timeout."
context: "user"
unit: "ms"
default: "0"
min: "0"
max: "2147483647"
url: "https://www.postgresql.org/docs/17/runtime-config-client.html#GUC-IDLE-IN-TRANSACTION-SESSION-TIMEOUT"
---

Terminate any session that has been idle (that is, waiting for a client query) within an open transaction for longer than the specified amount of time. If this value is specified without units, it is taken as milliseconds. A value of zero (the default) disables the timeout.

This option can be used to ensure that idle sessions do not hold locks for an unreasonable amount of time. Even when no significant locks are held, an open transaction prevents vacuuming away recently-dead tuples that may be visible only to this transaction; so remaining idle for a long time can contribute to table bloat. See [Routine Vacuuming](https://www.postgresql.org/docs/17/routine-vacuuming.html) for more details.
