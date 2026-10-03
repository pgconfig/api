---
name: "debug_print_plan"
version: "18"
type: "boolean"
category: "Reporting and Logging / What to Log"
short_desc: "Logs each query's execution plan."
context: "user"
default: "off"
url: "https://www.postgresql.org/docs/18/runtime-config-logging.html#GUC-DEBUG-PRINT-PARSE"
---

These parameters enable various debugging output to be emitted. When set, they print the resulting parse tree, the query rewriter output, or the execution plan for each executed query. These messages are emitted at `LOG` message level, so by default they will appear in the server log but will not be sent to the client. You can change that by adjusting [`client_min_messages`](https://www.postgresql.org/docs/18/runtime-config-client.html#GUC-CLIENT-MIN-MESSAGES) and/or [`log_min_messages`](https://www.postgresql.org/docs/18/runtime-config-logging.html#GUC-LOG-MIN-MESSAGES). These parameters are off by default.
