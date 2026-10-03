---
name: "client_min_messages"
version: "10"
type: "enum"
category: "Client Connection Defaults / Statement Behavior"
short_desc: "Sets the message levels that are sent to the client."
extra_desc: "Each level includes all the levels that follow it. The later the level, the fewer messages are sent."
context: "user"
default: "notice"
values: ["debug5", "debug4", "debug3", "debug2", "debug1", "log", "notice", "warning", "error"]
url: "https://www.postgresql.org/docs/10/runtime-config-client.html#GUC-CLIENT-MIN-MESSAGES"
---

Controls which [message levels](https://www.postgresql.org/docs/10/runtime-config-logging.html#RUNTIME-CONFIG-SEVERITY-LEVELS) are sent to the client. Valid values are `DEBUG5`, `DEBUG4`, `DEBUG3`, `DEBUG2`, `DEBUG1`, `LOG`, `NOTICE`, `WARNING`, and `ERROR`. Each level includes all the levels that follow it. The later the level, the fewer messages are sent. The default is `NOTICE`. Note that `LOG` has a different rank here than in [`log_min_messages`](https://www.postgresql.org/docs/10/runtime-config-logging.html#GUC-LOG-MIN-MESSAGES).

`INFO` level messages are always sent to the client.
