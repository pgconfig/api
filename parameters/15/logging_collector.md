---
name: "logging_collector"
version: "15"
type: "boolean"
category: "Reporting and Logging / Where to Log"
short_desc: "Start a subprocess to capture stderr, csvlog and/or jsonlog into log files."
context: "postmaster"
default: "off"
url: "https://www.postgresql.org/docs/15/runtime-config-logging.html#GUC-LOGGING-COLLECTOR"
---

This parameter enables the *logging collector*, which is a background process that captures log messages sent to `stderr` and redirects them into log files. This approach is often more useful than logging to syslog, since some types of messages might not appear in syslog output. (One common example is dynamic-linker failure messages; another is error messages produced by scripts such as `archive_command`.) This parameter can only be set at server start.

> [!NOTE]
> It is possible to log to `stderr` without using the logging collector; the log messages will just go to wherever the server's `stderr` is directed. However, that method is only suitable for low log volumes, since it provides no convenient way to rotate log files. Also, on some platforms not using the logging collector can result in lost or garbled log output, because multiple processes writing concurrently to the same log file can overwrite each other's output.

> [!NOTE]
> The logging collector is designed to never lose messages. This means that in case of extremely high load, server processes could be blocked while trying to send additional log messages when the collector has fallen behind. In contrast, syslog prefers to drop messages if it cannot write them, which means it may fail to log some messages in such cases but it will not block the rest of the system.
