---
name: "log_connections"
version: "13"
type: "boolean"
category: "Reporting and Logging / What to Log"
short_desc: "Logs each successful connection."
context: "superuser-backend"
default: "off"
url: "https://www.postgresql.org/docs/13/runtime-config-logging.html#GUC-LOG-CONNECTIONS"
---

Causes each attempted connection to the server to be logged, as well as successful completion of client authentication. Only superusers can change this parameter at session start, and it cannot be changed at all within a session. The default is `off`.

> [!NOTE]
> Some client programs, like psql, attempt to connect twice while determining if a password is required, so duplicate "connection received" messages do not necessarily indicate a problem.
