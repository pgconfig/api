---
name: "log_disconnections"
version: "9.1"
type: "boolean"
category: "Reporting and Logging / What to Log"
short_desc: "Logs end of a session, including duration."
context: "backend"
default: "off"
url: "https://www.postgresql.org/docs/9.1/runtime-config-logging.html#GUC-LOG-DISCONNECTIONS"
---

This outputs a line in the server log similar to `log_connections` but at session termination, and includes the duration of the session. This is off by default. This parameter cannot be changed after session start.
