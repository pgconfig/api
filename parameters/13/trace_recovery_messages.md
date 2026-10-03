---
name: "trace_recovery_messages"
version: "13"
type: "enum"
category: "Developer Options"
short_desc: "Enables logging of recovery-related debugging information."
extra_desc: "Each level includes all the levels that follow it. The later the level, the fewer messages are sent."
context: "sighup"
default: "log"
values: ["debug5", "debug4", "debug3", "debug2", "debug1", "log", "notice", "warning", "error"]
url: "https://www.postgresql.org/docs/13/runtime-config-developer.html#GUC-TRACE-RECOVERY-MESSAGES"
---

Enables logging of recovery-related debugging output that otherwise would not be logged. This parameter allows the user to override the normal setting of [`log_min_messages`](https://www.postgresql.org/docs/13/runtime-config-logging.html#GUC-LOG-MIN-MESSAGES), but only for specific messages. This is intended for use in debugging Hot Standby. Valid values are `DEBUG5`, `DEBUG4`, `DEBUG3`, `DEBUG2`, `DEBUG1`, and `LOG`. The default, `LOG`, does not affect logging decisions at all. The other values cause recovery-related debug messages of that priority or higher to be logged as though they had `LOG` priority; for common settings of `log_min_messages` this results in unconditionally sending them to the server log. This parameter can only be set in the `postgresql.conf` file or on the server command line.
