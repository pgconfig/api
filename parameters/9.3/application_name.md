---
name: "application_name"
version: "9.3"
type: "string"
category: "Reporting and Logging / What to Log"
short_desc: "Sets the application name to be reported in statistics and logs."
context: "user"
default: ""
url: "https://www.postgresql.org/docs/9.3/runtime-config-logging.html#GUC-APPLICATION-NAME"
---

The `application_name` can be any string of less than `NAMEDATALEN` characters (64 characters in a standard build). It is typically set by an application upon connection to the server. The name will be displayed in the `pg_stat_activity` view and included in CSV log entries. It can also be included in regular log entries via the [`log_line_prefix`](https://www.postgresql.org/docs/9.3/runtime-config-logging.html#GUC-LOG-LINE-PREFIX) parameter. Only printable ASCII characters may be used in the `application_name` value. Other characters will be replaced with question marks (`?`).
