---
name: "syslog_ident"
version: "9.5"
type: "string"
category: "Reporting and Logging / Where to Log"
short_desc: "Sets the program name used to identify PostgreSQL messages in syslog."
context: "sighup"
default: "postgres"
url: "https://www.postgresql.org/docs/9.5/runtime-config-logging.html#GUC-SYSLOG-IDENT"
---

When logging to syslog is enabled, this parameter determines the program name used to identify PostgreSQL messages in syslog logs. The default is `postgres`. This parameter can only be set in the `postgresql.conf` file or on the server command line.
