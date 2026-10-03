---
name: "syslog_split_messages"
version: "9.6"
type: "boolean"
category: "Reporting and Logging / Where to Log"
short_desc: "Split messages sent to syslog by lines and to fit into 1024 bytes."
context: "sighup"
default: "on"
url: "https://www.postgresql.org/docs/9.6/runtime-config-logging.html#GUC-SYSLOG-SPLIT-MESSAGES"
---

When logging to syslog is enabled, this parameter determines how messages are delivered to syslog. When on (the default), messages are split by lines, and long lines are split so that they will fit into 1024 bytes, which is a typical size limit for traditional syslog implementations. When off, PostgreSQL server log messages are delivered to the syslog service as is, and it is up to the syslog service to cope with the potentially bulky messages.

If syslog is ultimately logging to a text file, then the effect will be the same either way, and it is best to leave the setting on, since most syslog implementations either cannot handle large messages or would need to be specially configured to handle them. But if syslog is ultimately writing into some other medium, it might be necessary or more useful to keep messages logically together.

This parameter can only be set in the `postgresql.conf` file or on the server command line.
