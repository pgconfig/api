---
name: "send_abort_for_crash"
version: "18"
type: "boolean"
category: "Developer Options"
short_desc: "Send SIGABRT not SIGQUIT to child processes after backend crash."
context: "sighup"
default: "off"
url: "https://www.postgresql.org/docs/18/runtime-config-developer.html#GUC-SEND-ABORT-FOR-CRASH"
---

By default, after a backend crash the postmaster will stop remaining child processes by sending them `SIGQUIT` signals, which permits them to exit more-or-less gracefully. When this option is set to `on`, `SIGABRT` is sent instead. That normally results in production of a core dump file for each such child process. This can be handy for investigating the states of other processes after a crash. It can also consume lots of disk space in the event of repeated crashes, so do not enable this on systems you are not monitoring carefully. Beware that no support exists for cleaning up the core file(s) automatically. This parameter can only be set in the `postgresql.conf` file or on the server command line.
