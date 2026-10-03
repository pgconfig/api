---
name: "log_rotation_size"
version: "17"
type: "integer"
category: "Reporting and Logging / Where to Log"
short_desc: "Sets the maximum size a log file can reach before being rotated."
context: "sighup"
unit: "kB"
default: "10240"
min: "0"
max: "2097151"
url: "https://www.postgresql.org/docs/17/runtime-config-logging.html#GUC-LOG-ROTATION-SIZE"
---

When `logging_collector` is enabled, this parameter determines the maximum size of an individual log file. After this amount of data has been emitted into a log file, a new log file will be created. If this value is specified without units, it is taken as kilobytes. The default is 10 megabytes. Set to zero to disable size-based creation of new log files. This parameter can only be set in the `postgresql.conf` file or on the server command line.
