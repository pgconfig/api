---
name: "log_replication_commands"
version: "18"
type: "boolean"
category: "Reporting and Logging / What to Log"
short_desc: "Logs each replication command."
context: "superuser"
default: "off"
url: "https://www.postgresql.org/docs/18/runtime-config-logging.html#GUC-LOG-REPLICATION-COMMANDS"
---

Causes each replication command and `walsender` process's replication slot acquisition/release to be logged in the server log. See [Streaming Replication Protocol](https://www.postgresql.org/docs/18/protocol-replication.html) for more information about replication command. The default value is `off`. Only superusers and users with the appropriate `SET` privilege can change this setting.
