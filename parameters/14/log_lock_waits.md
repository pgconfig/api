---
name: "log_lock_waits"
version: "14"
type: "boolean"
category: "Reporting and Logging / What to Log"
short_desc: "Logs long lock waits."
context: "superuser"
default: "off"
url: "https://www.postgresql.org/docs/14/runtime-config-logging.html#GUC-LOG-LOCK-WAITS"
---

Controls whether a log message is produced when a session waits longer than [`deadlock_timeout`](https://www.postgresql.org/docs/14/runtime-config-locks.html#GUC-DEADLOCK-TIMEOUT) to acquire a lock. This is useful in determining if lock waits are causing poor performance. The default is `off`. Only superusers can change this setting.
