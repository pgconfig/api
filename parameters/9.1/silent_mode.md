---
name: "silent_mode"
version: "9.1"
type: "boolean"
category: "Reporting and Logging / Where to Log"
short_desc: "Runs the server silently."
extra_desc: "If this parameter is set, the server will automatically run in the background and any controlling terminals are dissociated."
context: "postmaster"
default: "off"
url: "https://www.postgresql.org/docs/9.1/runtime-config-logging.html#GUC-SILENT-MODE"
---

Runs the server silently. If this parameter is set, the server will automatically run in background and disassociate from the controlling terminal. This parameter can only be set at server start.

> [!CAUTION]
> When this parameter is set, the server's standard output and standard error are redirected to the file `postmaster.log` within the data directory. There is no provision for rotating this file, so it will grow indefinitely unless server log output is redirected elsewhere by other settings. It is recommended that `log_destination` be set to `syslog` or that `logging_collector` be enabled when using this option. Even with those measures, errors reported early during startup may appear in `postmaster.log` rather than the normal log destination.
