---
name: "log_min_duration_statement"
version: "18"
type: "integer"
category: "Reporting and Logging / When to Log"
short_desc: "Sets the minimum execution time above which all statements will be logged."
extra_desc: "-1 disables logging statement durations. 0 means log all statement durations."
context: "superuser"
unit: "ms"
default: "-1"
min: "-1"
max: "2147483647"
url: "https://www.postgresql.org/docs/18/runtime-config-logging.html#GUC-LOG-MIN-DURATION-STATEMENT"
---

Causes the duration of each completed statement to be logged if the statement ran for at least the specified amount of time. For example, if you set it to `250ms` then all SQL statements that run 250ms or longer will be logged. Enabling this parameter can be helpful in tracking down unoptimized queries in your applications. If this value is specified without units, it is taken as milliseconds. Setting this to zero prints all statement durations. `-1` (the default) disables logging statement durations. Only superusers and users with the appropriate `SET` privilege can change this setting.

This overrides [`log_min_duration_sample`](https://www.postgresql.org/docs/18/runtime-config-logging.html#GUC-LOG-MIN-DURATION-SAMPLE), meaning that queries with duration exceeding this setting are not subject to sampling and are always logged.

For clients using extended query protocol, durations of the Parse, Bind, and Execute steps are logged independently.

> [!NOTE]
> When using this option together with [`log_statement`](https://www.postgresql.org/docs/18/runtime-config-logging.html#GUC-LOG-STATEMENT), the text of statements that are logged because of `log_statement` will not be repeated in the duration log message. If you are not using syslog, it is recommended that you log the PID or session ID using [`log_line_prefix`](https://www.postgresql.org/docs/18/runtime-config-logging.html#GUC-LOG-LINE-PREFIX) so that you can link the statement message to the later duration message using the process ID or session ID.
