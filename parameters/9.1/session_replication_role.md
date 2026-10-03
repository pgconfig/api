---
name: "session_replication_role"
version: "9.1"
type: "enum"
category: "Client Connection Defaults / Statement Behavior"
short_desc: "Sets the session's behavior for triggers and rewrite rules."
context: "superuser"
default: "origin"
values: ["origin", "replica", "local"]
url: "https://www.postgresql.org/docs/9.1/runtime-config-client.html#GUC-SESSION-REPLICATION-ROLE"
---

Controls firing of replication-related triggers and rules for the current session. Setting this variable requires superuser privilege and results in discarding any previously cached query plans. Possible values are `origin` (the default), `replica` and `local`. See [ALTER TABLE](https://www.postgresql.org/docs/9.1/sql-altertable.html) for more information.
