---
name: "log_connections"
version: "9.2"
type: "boolean"
category: "Reporting and Logging / What to Log"
short_desc: "Logs each successful connection."
context: "backend"
default: "off"
url: "https://www.postgresql.org/docs/9.2/runtime-config-logging.html#GUC-LOG-CONNECTIONS"
---

Causes each attempted connection to the server to be logged, as well as successful completion of client authentication. This parameter cannot be changed after session start. The default is off.

> [!NOTE]
> Some client programs, like psql, attempt to connect twice while determining if a password is required, so duplicate "connection received" messages do not necessarily indicate a problem.
