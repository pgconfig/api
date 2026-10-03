---
name: "log_executor_stats"
version: "17"
type: "boolean"
category: "Statistics / Monitoring"
short_desc: "Writes executor performance statistics to the server log."
context: "superuser"
default: "off"
url: "https://www.postgresql.org/docs/17/runtime-config-statistics.html#GUC-LOG-STATEMENT-STATS"
---

For each query, output performance statistics of the respective module to the server log. This is a crude profiling instrument, similar to the Unix `getrusage()` operating system facility. `log_statement_stats` reports total statement statistics, while the others report per-module statistics. `log_statement_stats` cannot be enabled together with any of the per-module options. All of these options are disabled by default. Only superusers and users with the appropriate `SET` privilege can change these settings.
