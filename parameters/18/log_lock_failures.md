---
name: "log_lock_failures"
version: "18"
type: "boolean"
category: "Reporting and Logging / What to Log"
short_desc: "Logs lock failures."
context: "superuser"
default: "off"
url: "https://www.postgresql.org/docs/18/runtime-config-logging.html#GUC-LOG-LOCK-FAILURES"
---

Controls whether a detailed log message is produced when a lock acquisition fails. This is useful for analyzing the causes of lock failures. Currently, only lock failures due to `SELECT NOWAIT` is supported. The default is `off`. Only superusers and users with the appropriate `SET` privilege can change this setting.
