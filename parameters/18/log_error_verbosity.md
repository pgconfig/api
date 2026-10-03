---
name: "log_error_verbosity"
version: "18"
type: "enum"
category: "Reporting and Logging / What to Log"
short_desc: "Sets the verbosity of logged messages."
context: "superuser"
default: "default"
values: ["terse", "default", "verbose"]
url: "https://www.postgresql.org/docs/18/runtime-config-logging.html#GUC-LOG-ERROR-VERBOSITY"
---

Controls the amount of detail written in the server log for each message that is logged. Valid values are `TERSE`, `DEFAULT`, and `VERBOSE`, each adding more fields to displayed messages. `TERSE` excludes the logging of `DETAIL`, `HINT`, `QUERY`, and `CONTEXT` error information. `VERBOSE` output includes the `SQLSTATE` error code (see also [PostgreSQL Error Codes](https://www.postgresql.org/docs/18/errcodes-appendix.html)) and the source code file name, function name, and line number that generated the error. Only superusers and users with the appropriate `SET` privilege can change this setting.
