---
name: "log_min_messages"
version: "19"
type: "string"
category: "Reporting and Logging / When to Log"
short_desc: "Sets the message levels that are logged."
extra_desc: "Each level includes all the levels that follow it. The later the level, the fewer messages are sent."
context: "superuser"
default: "warning"
url: "https://www.postgresql.org/docs/19/runtime-config-logging.html#GUC-LOG-MIN-MESSAGES"
---

Controls which [message levels](https://www.postgresql.org/docs/19/runtime-config-logging.html#RUNTIME-CONFIG-SEVERITY-LEVELS) are written to the server log. The value is a comma-separated list of zero or more `process type:level` entries and exactly one mandatory `level` entry, which becomes the default for process types not listed. Valid process types are listed in the table below.

- `archiver`
- `autovacuum`
- `backend`
- `bgworker`
- `bgwriter`
- `checkpointer`
- `ioworker`
- `postmaster`
- `slotsyncworker`
- `startup`
- `syslogger`
- `walreceiver`
- `walsender`
- `walsummarizer`
- `walwriter`

Valid `level` values are `DEBUG5`, `DEBUG4`, `DEBUG3`, `DEBUG2`, `DEBUG1`, `INFO`, `NOTICE`, `WARNING`, `ERROR`, `LOG`, `FATAL`, and `PANIC`. Each level includes all the levels that follow it. The later the level, the fewer messages are sent to the log. The default is `WARNING`, which applies that level to all process types. Note that `LOG` has a different rank here than in [`client_min_messages`](https://www.postgresql.org/docs/19/runtime-config-client.html#GUC-CLIENT-MIN-MESSAGES). Only superusers and users with the appropriate `SET` privilege can change this setting.

Example: To log `walsender` and `autovacuum` at level `DEBUG1` and everything else at `ERROR`, set `log_min_messages` to `error, walsender:debug1, autovacuum:debug1`.
