---
name: "syslog_facility"
version: "9.4"
type: "enum"
category: "Reporting and Logging / Where to Log"
short_desc: "Sets the syslog \"facility\" to be used when syslog enabled."
context: "sighup"
default: "local0"
values: ["local0", "local1", "local2", "local3", "local4", "local5", "local6", "local7"]
url: "https://www.postgresql.org/docs/9.4/runtime-config-logging.html#GUC-SYSLOG-FACILITY"
---

When logging to syslog is enabled, this parameter determines the syslog "facility" to be used. You can choose from `LOCAL0`, `LOCAL1`, `LOCAL2`, `LOCAL3`, `LOCAL4`, `LOCAL5`, `LOCAL6`, `LOCAL7`; the default is `LOCAL0`. See also the documentation of your system's syslog daemon. This parameter can only be set in the `postgresql.conf` file or on the server command line.
