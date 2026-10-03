---
name: "trace_notify"
version: "14"
type: "boolean"
category: "Developer Options"
short_desc: "Generates debugging output for LISTEN and NOTIFY."
context: "user"
default: "off"
url: "https://www.postgresql.org/docs/14/runtime-config-developer.html#GUC-TRACE-NOTIFY"
---

Generates a great amount of debugging output for the `LISTEN` and `NOTIFY` commands. [`client_min_messages`](https://www.postgresql.org/docs/14/runtime-config-client.html#GUC-CLIENT-MIN-MESSAGES) or [`log_min_messages`](https://www.postgresql.org/docs/14/runtime-config-logging.html#GUC-LOG-MIN-MESSAGES) must be `DEBUG1` or lower to send this output to the client or server logs, respectively.
