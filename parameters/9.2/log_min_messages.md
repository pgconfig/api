---
name: "log_min_messages"
version: "9.2"
type: "enum"
category: "Reporting and Logging / When to Log"
short_desc: "Sets the message levels that are logged."
extra_desc: "Each level includes all the levels that follow it. The later the level, the fewer messages are sent."
context: "superuser"
default: "warning"
values: ["debug5", "debug4", "debug3", "debug2", "debug1", "info", "notice", "warning", "error", "log", "fatal", "panic"]
url: "https://www.postgresql.org/docs/9.2/runtime-config-logging.html#GUC-LOG-MIN-MESSAGES"
---

Controls which message levels are written to the server log. Valid values are `DEBUG5`, `DEBUG4`, `DEBUG3`, `DEBUG2`, `DEBUG1`, `INFO`, `NOTICE`, `WARNING`, `ERROR`, `LOG`, `FATAL`, and `PANIC`. Each level includes all the levels that follow it. The later the level, the fewer messages are sent to the log. The default is `WARNING`. Note that `LOG` has a different rank here than in `client_min_messages`. Only superusers can change this setting.
