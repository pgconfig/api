---
name: "log_rotation_age"
version: "10"
type: "integer"
category: "Reporting and Logging / Where to Log"
short_desc: "Automatic log file rotation will occur after N minutes."
context: "sighup"
unit: "min"
default: "1440"
min: "0"
max: "35791394"
url: "https://www.postgresql.org/docs/10/runtime-config-logging.html#GUC-LOG-ROTATION-AGE"
---

When `logging_collector` is enabled, this parameter determines the maximum lifetime of an individual log file. After this many minutes have elapsed, a new log file will be created. Set to zero to disable time-based creation of new log files. This parameter can only be set in the `postgresql.conf` file or on the server command line.
