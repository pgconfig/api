---
name: "lock_timeout"
version: "9.5"
type: "integer"
category: "Client Connection Defaults / Statement Behavior"
short_desc: "Sets the maximum allowed duration of any wait for a lock."
extra_desc: "A value of 0 turns off the timeout."
context: "user"
unit: "ms"
default: "0"
min: "0"
max: "2147483647"
url: "https://www.postgresql.org/docs/9.5/runtime-config-client.html#GUC-LOCK-TIMEOUT"
---

Abort any statement that waits longer than the specified number of milliseconds while attempting to acquire a lock on a table, index, row, or other database object. The time limit applies separately to each lock acquisition attempt. The limit applies both to explicit locking requests (such as `LOCK TABLE`, or `SELECT FOR UPDATE` without `NOWAIT`) and to implicitly-acquired locks. A value of zero (the default) turns this off.

Unlike `statement_timeout`, this timeout can only occur while waiting for locks. Note that if `statement_timeout` is nonzero, it is rather pointless to set `lock_timeout` to the same or larger value, since the statement timeout would always trigger first. If `log_min_error_statement` is set to `ERROR` or lower, the statement that timed out will be logged.

Setting `lock_timeout` in `postgresql.conf` is not recommended because it would affect all sessions.
