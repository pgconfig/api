---
name: "log_min_error_statement"
version: "16"
type: "enum"
category: "Reporting and Logging / When to Log"
short_desc: "Causes all statements generating error at or above this level to be logged."
extra_desc: "Each level includes all the levels that follow it. The later the level, the fewer messages are sent."
context: "superuser"
default: "error"
values: ["debug5", "debug4", "debug3", "debug2", "debug1", "info", "notice", "warning", "error", "log", "fatal", "panic"]
url: "https://www.postgresql.org/docs/16/runtime-config-logging.html#GUC-LOG-MIN-ERROR-STATEMENT"
---

Controls which SQL statements that cause an error condition are recorded in the server log. The current SQL statement is included in the log entry for any message of the specified [severity](https://www.postgresql.org/docs/16/runtime-config-logging.html#RUNTIME-CONFIG-SEVERITY-LEVELS) or higher. Valid values are `DEBUG5`, `DEBUG4`, `DEBUG3`, `DEBUG2`, `DEBUG1`, `INFO`, `NOTICE`, `WARNING`, `ERROR`, `LOG`, `FATAL`, and `PANIC`. The default is `ERROR`, which means statements causing errors, log messages, fatal errors, or panics will be logged. To effectively turn off logging of failing statements, set this parameter to `PANIC`. Only superusers and users with the appropriate `SET` privilege can change this setting.
