---
name: "log_disconnections"
version: "16"
type: "boolean"
category: "Reporting and Logging / What to Log"
short_desc: "Logs end of a session, including duration."
context: "superuser-backend"
default: "off"
url: "https://www.postgresql.org/docs/16/runtime-config-logging.html#GUC-LOG-DISCONNECTIONS"
---

Causes session terminations to be logged. The log output provides information similar to `log_connections`, plus the duration of the session. Only superusers and users with the appropriate `SET` privilege can change this parameter at session start, and it cannot be changed at all within a session. The default is `off`.
