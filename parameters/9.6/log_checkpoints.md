---
name: "log_checkpoints"
version: "9.6"
type: "boolean"
category: "Reporting and Logging / What to Log"
short_desc: "Logs each checkpoint."
context: "sighup"
default: "off"
url: "https://www.postgresql.org/docs/9.6/runtime-config-logging.html#GUC-LOG-CHECKPOINTS"
---

Causes checkpoints and restartpoints to be logged in the server log. Some statistics are included in the log messages, including the number of buffers written and the time spent writing them. This parameter can only be set in the `postgresql.conf` file or on the server command line. The default is off.
