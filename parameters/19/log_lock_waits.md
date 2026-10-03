---
name: "log_lock_waits"
version: "19"
type: "boolean"
category: "Reporting and Logging / What to Log"
short_desc: "Logs long lock waits."
context: "superuser"
default: "on"
url: "https://www.postgresql.org/docs/19/runtime-config-logging.html#GUC-LOG-LOCK-WAITS"
---

Controls whether a log message is produced when a session waits longer than [`deadlock_timeout`](https://www.postgresql.org/docs/19/runtime-config-locks.html#GUC-DEADLOCK-TIMEOUT) to acquire a lock. This is useful in determining if lock waits are causing poor performance. The default is `on`. Only superusers and users with the appropriate `SET` privilege can change this setting.
