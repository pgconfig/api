---
name: "log_transaction_sample_rate"
version: "12"
type: "floating point"
category: "Reporting and Logging / When to Log"
short_desc: "Set the fraction of transactions to log for new transactions."
extra_desc: "Logs all statements from a fraction of transactions. Use a value between 0.0 (never log) and 1.0 (log all statements for all transactions)."
context: "superuser"
default: "0"
min: "0"
max: "1"
url: "https://www.postgresql.org/docs/12/runtime-config-logging.html#GUC-LOG-TRANSACTION-SAMPLE-RATE"
---

Set the fraction of transactions whose statements are all logged, in addition to statements logged for other reasons. It applies to each new transaction regardless of its statements' durations. The default is `0`, meaning not to log statements from any additional transaction. Setting this to `1` logs all statements for all transactions. `log_transaction_sample_rate` is helpful to track a sample of transaction. Only superusers can change this setting.

> [!NOTE]
> Like all statement-logging options, this option can add significant overhead.
