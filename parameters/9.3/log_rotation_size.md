---
name: "log_rotation_size"
version: "9.3"
type: "integer"
category: "Reporting and Logging / Where to Log"
short_desc: "Automatic log file rotation will occur after N kilobytes."
context: "sighup"
unit: "kB"
default: "10240"
min: "0"
max: "2097151"
url: "https://www.postgresql.org/docs/9.3/runtime-config-logging.html#GUC-LOG-ROTATION-SIZE"
---

When `logging_collector` is enabled, this parameter determines the maximum size of an individual log file. After this many kilobytes have been emitted into a log file, a new log file will be created. Set to zero to disable size-based creation of new log files. This parameter can only be set in the `postgresql.conf` file or on the server command line.
