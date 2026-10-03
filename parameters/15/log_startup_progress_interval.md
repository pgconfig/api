---
name: "log_startup_progress_interval"
version: "15"
type: "integer"
category: "Reporting and Logging / When to Log"
short_desc: "Time between progress updates for long-running startup operations."
extra_desc: "0 turns this feature off."
context: "sighup"
unit: "ms"
default: "10000"
min: "0"
max: "2147483647"
url: "https://www.postgresql.org/docs/15/runtime-config-logging.html#GUC-LOG-STARTUP-PROGRESS-INTERVAL"
---

Sets the amount of time after which the startup process will log a message about a long-running operation that is still in progress, as well as the interval between further progress messages for that operation. The default is 10 seconds. A setting of `0` disables the feature. If this value is specified without units, it is taken as milliseconds. This setting is applied separately to each operation. This parameter can only be set in the `postgresql.conf` file or on the server command line.

For example, if syncing the data directory takes 25 seconds and thereafter resetting unlogged relations takes 8 seconds, and if this setting has the default value of 10 seconds, then a messages will be logged for syncing the data directory after it has been in progress for 10 seconds and again after it has been in progress for 20 seconds, but nothing will be logged for resetting unlogged relations.
