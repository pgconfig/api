---
name: "client_min_messages"
version: "9.1"
type: "enum"
category: "Reporting and Logging / When to Log"
short_desc: "Sets the message levels that are sent to the client."
extra_desc: "Each level includes all the levels that follow it. The later the level, the fewer messages are sent."
context: "user"
default: "notice"
values: ["debug5", "debug4", "debug3", "debug2", "debug1", "log", "notice", "warning", "error"]
url: "https://www.postgresql.org/docs/9.1/runtime-config-logging.html#GUC-CLIENT-MIN-MESSAGES"
---

Controls which message levels are sent to the client. Valid values are `DEBUG5`, `DEBUG4`, `DEBUG3`, `DEBUG2`, `DEBUG1`, `LOG`, `NOTICE`, `WARNING`, `ERROR`, `FATAL`, and `PANIC`. Each level includes all the levels that follow it. The later the level, the fewer messages are sent. The default is `NOTICE`. Note that `LOG` has a different rank here than in `log_min_messages`.
