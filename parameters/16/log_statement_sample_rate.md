---
name: "log_statement_sample_rate"
version: "16"
type: "floating point"
category: "Reporting and Logging / When to Log"
short_desc: "Fraction of statements exceeding log_min_duration_sample to be logged."
extra_desc: "Use a value between 0.0 (never log) and 1.0 (always log)."
context: "superuser"
default: "1"
min: "0"
max: "1"
url: "https://www.postgresql.org/docs/16/runtime-config-logging.html#GUC-LOG-STATEMENT-SAMPLE-RATE"
---

Determines the fraction of statements with duration exceeding [`log_min_duration_sample`](https://www.postgresql.org/docs/16/runtime-config-logging.html#GUC-LOG-MIN-DURATION-SAMPLE) that will be logged. Sampling is stochastic, for example `0.5` means there is statistically one chance in two that any given statement will be logged. The default is `1.0`, meaning to log all sampled statements. Setting this to zero disables sampled statement-duration logging, the same as setting `log_min_duration_sample` to `-1`. Only superusers and users with the appropriate `SET` privilege can change this setting.
