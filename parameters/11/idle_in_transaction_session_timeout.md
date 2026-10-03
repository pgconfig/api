---
name: "idle_in_transaction_session_timeout"
version: "11"
type: "integer"
category: "Client Connection Defaults / Statement Behavior"
short_desc: "Sets the maximum allowed duration of any idling transaction."
extra_desc: "A value of 0 turns off the timeout."
context: "user"
unit: "ms"
default: "0"
min: "0"
max: "2147483647"
url: "https://www.postgresql.org/docs/11/runtime-config-client.html#GUC-IDLE-IN-TRANSACTION-SESSION-TIMEOUT"
---

Terminate any session with an open transaction that has been idle for longer than the specified duration in milliseconds. This allows any locks held by that session to be released and the connection slot to be reused; it also allows tuples visible only to this transaction to be vacuumed. See [Routine Vacuuming](https://www.postgresql.org/docs/11/routine-vacuuming.html) for more details about this.

The default value of 0 disables this feature.
