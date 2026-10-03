---
name: "log_rotation_age"
version: "18"
type: "integer"
category: "Reporting and Logging / Where to Log"
short_desc: "Sets the amount of time to wait before forcing log file rotation."
extra_desc: "0 disables time-based creation of new log files."
context: "sighup"
unit: "min"
default: "1440"
min: "0"
max: "35791394"
url: "https://www.postgresql.org/docs/18/runtime-config-logging.html#GUC-LOG-ROTATION-AGE"
---

When `logging_collector` is enabled, this parameter determines the maximum amount of time to use an individual log file, after which a new log file will be created. If this value is specified without units, it is taken as minutes. The default is 24 hours. Set to zero to disable time-based creation of new log files. This parameter can only be set in the `postgresql.conf` file or on the server command line.
