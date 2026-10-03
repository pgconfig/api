---
name: "idle_session_timeout"
version: "18"
type: "integer"
category: "Client Connection Defaults / Statement Behavior"
short_desc: "Sets the maximum allowed idle time between queries, when not in a transaction."
extra_desc: "0 disables the timeout."
context: "user"
unit: "ms"
default: "0"
min: "0"
max: "2147483647"
url: "https://www.postgresql.org/docs/18/runtime-config-client.html#GUC-IDLE-SESSION-TIMEOUT"
---

Terminate any session that has been idle (that is, waiting for a client query), but not within an open transaction, for longer than the specified amount of time. If this value is specified without units, it is taken as milliseconds. A value of zero (the default) disables the timeout.

Unlike the case with an open transaction, an idle session without a transaction imposes no large costs on the server, so there is less need to enable this timeout than `idle_in_transaction_session_timeout`.

Be wary of enforcing this timeout on connections made through connection-pooling software or other middleware, as such a layer may not react well to unexpected connection closure. It may be helpful to enable this timeout only for interactive sessions, perhaps by applying it only to particular users.
