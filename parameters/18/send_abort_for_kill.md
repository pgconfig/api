---
name: "send_abort_for_kill"
version: "18"
type: "boolean"
category: "Developer Options"
short_desc: "Send SIGABRT not SIGKILL to stuck child processes."
context: "sighup"
default: "off"
url: "https://www.postgresql.org/docs/18/runtime-config-developer.html#GUC-SEND-ABORT-FOR-KILL"
---

By default, after attempting to stop a child process with `SIGQUIT`, the postmaster will wait five seconds and then send `SIGKILL` to force immediate termination. When this option is set to `on`, `SIGABRT` is sent instead of `SIGKILL`. That normally results in production of a core dump file for each such child process. This can be handy for investigating the states of "stuck" child processes. It can also consume lots of disk space in the event of repeated crashes, so do not enable this on systems you are not monitoring carefully. Beware that no support exists for cleaning up the core file(s) automatically. This parameter can only be set in the `postgresql.conf` file or on the server command line.
