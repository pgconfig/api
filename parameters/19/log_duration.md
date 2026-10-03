---
name: "log_duration"
version: "19"
type: "boolean"
category: "Reporting and Logging / What to Log"
short_desc: "Logs the duration of each completed SQL statement."
context: "superuser"
default: "off"
url: "https://www.postgresql.org/docs/19/runtime-config-logging.html#GUC-LOG-DURATION"
---

Causes the duration of every completed statement to be logged. The default is `off`. Only superusers and users with the appropriate `SET` privilege can change this setting.

For clients using extended query protocol, durations of the Parse, Bind, and Execute steps are logged independently.

> [!NOTE]
> The difference between enabling `log_duration` and setting [`log_min_duration_statement`](https://www.postgresql.org/docs/19/runtime-config-logging.html#GUC-LOG-MIN-DURATION-STATEMENT) to zero is that exceeding `log_min_duration_statement` forces the text of the query to be logged, but this option doesn't. Thus, if `log_duration` is `on` and `log_min_duration_statement` has a positive value, all durations are logged but the query text is included only for statements exceeding the threshold. This behavior can be useful for gathering statistics in high-load installations.
