---
name: "idle_in_transaction_session_timeout"
version: "13"
type: "integer"
category: "Client Connection Defaults / Statement Behavior"
short_desc: "Sets the maximum allowed duration of any idling transaction."
extra_desc: "A value of 0 turns off the timeout."
context: "user"
unit: "ms"
default: "0"
min: "0"
max: "2147483647"
url: "https://www.postgresql.org/docs/13/runtime-config-client.html#GUC-IDLE-IN-TRANSACTION-SESSION-TIMEOUT"
---

Terminate any session with an open transaction that has been idle for longer than the specified amount of time. This allows any locks held by that session to be released and the connection slot to be reused; it also allows tuples visible only to this transaction to be vacuumed. See [Routine Vacuuming](https://www.postgresql.org/docs/13/routine-vacuuming.html) for more details about this.

If this value is specified without units, it is taken as milliseconds. A value of zero (the default) disables the timeout.
