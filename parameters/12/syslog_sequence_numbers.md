---
name: "syslog_sequence_numbers"
version: "12"
type: "boolean"
category: "Reporting and Logging / Where to Log"
short_desc: "Add sequence number to syslog messages to avoid duplicate suppression."
context: "sighup"
default: "on"
url: "https://www.postgresql.org/docs/12/runtime-config-logging.html#GUC-SYSLOG-SEQUENCE-NUMBERS"
---

When logging to syslog and this is on (the default), then each message will be prefixed by an increasing sequence number (such as `[2]`). This circumvents the "--- last message repeated N times ---" suppression that many syslog implementations perform by default. In more modern syslog implementations, repeated message suppression can be configured (for example, `$RepeatedMsgReduction` in rsyslog), so this might not be necessary. Also, you could turn this off if you actually want to suppress repeated messages.

This parameter can only be set in the `postgresql.conf` file or on the server command line.
