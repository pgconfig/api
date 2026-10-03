---
name: "statement_timeout"
version: "9.6"
type: "integer"
category: "Client Connection Defaults / Statement Behavior"
short_desc: "Sets the maximum allowed duration of any statement."
extra_desc: "A value of 0 turns off the timeout."
context: "user"
unit: "ms"
default: "0"
min: "0"
max: "2147483647"
url: "https://www.postgresql.org/docs/9.6/runtime-config-client.html#GUC-STATEMENT-TIMEOUT"
---

Abort any statement that takes more than the specified number of milliseconds, starting from the time the command arrives at the server from the client. If `log_min_error_statement` is set to `ERROR` or lower, the statement that timed out will also be logged. A value of zero (the default) turns this off.

Setting `statement_timeout` in `postgresql.conf` is not recommended because it would affect all sessions.
