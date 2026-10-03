---
name: "log_recovery_conflict_waits"
version: "18"
type: "boolean"
category: "Reporting and Logging / What to Log"
short_desc: "Logs standby recovery conflict waits."
context: "sighup"
default: "off"
url: "https://www.postgresql.org/docs/18/runtime-config-logging.html#GUC-LOG-RECOVERY-CONFLICT-WAITS"
---

Controls whether a log message is produced when the startup process waits longer than `deadlock_timeout` for recovery conflicts. This is useful in determining if recovery conflicts prevent the recovery from applying WAL.

The default is `off`. This parameter can only be set in the `postgresql.conf` file or on the server command line.
