---
name: "log_directory"
version: "9.2"
type: "string"
category: "Reporting and Logging / Where to Log"
short_desc: "Sets the destination directory for log files."
extra_desc: "Can be specified as relative to the data directory or as absolute path."
context: "sighup"
default: "pg_log"
url: "https://www.postgresql.org/docs/9.2/runtime-config-logging.html#GUC-LOG-DIRECTORY"
---

When `logging_collector` is enabled, this parameter determines the directory in which log files will be created. It can be specified as an absolute path, or relative to the cluster data directory. This parameter can only be set in the `postgresql.conf` file or on the server command line. The default is `pg_log`.
