---
name: "log_transaction_sample_rate"
version: "15"
type: "floating point"
category: "Reporting and Logging / When to Log"
short_desc: "Sets the fraction of transactions from which to log all statements."
extra_desc: "Use a value between 0.0 (never log) and 1.0 (log all statements for all transactions)."
context: "superuser"
default: "0"
min: "0"
max: "1"
url: "https://www.postgresql.org/docs/15/runtime-config-logging.html#GUC-LOG-TRANSACTION-SAMPLE-RATE"
---

Sets the fraction of transactions whose statements are all logged, in addition to statements logged for other reasons. It applies to each new transaction regardless of its statements' durations. Sampling is stochastic, for example `0.1` means there is statistically one chance in ten that any given transaction will be logged. `log_transaction_sample_rate` can be helpful to construct a sample of transactions. The default is `0`, meaning not to log statements from any additional transactions. Setting this to `1` logs all statements of all transactions. Only superusers and users with the appropriate `SET` privilege can change this setting.

> [!NOTE]
> Like all statement-logging options, this option can add significant overhead.
