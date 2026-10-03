---
name: "session_replication_role"
version: "13"
type: "enum"
category: "Client Connection Defaults / Statement Behavior"
short_desc: "Sets the session's behavior for triggers and rewrite rules."
context: "superuser"
default: "origin"
values: ["origin", "replica", "local"]
url: "https://www.postgresql.org/docs/13/runtime-config-client.html#GUC-SESSION-REPLICATION-ROLE"
---

Controls firing of replication-related triggers and rules for the current session. Setting this variable requires superuser privilege and results in discarding any previously cached query plans. Possible values are `origin` (the default), `replica` and `local`.

The intended use of this setting is that logical replication systems set it to `replica` when they are applying replicated changes. The effect of that will be that triggers and rules (that have not been altered from their default configuration) will not fire on the replica. See the [ALTER TABLE](https://www.postgresql.org/docs/13/sql-altertable.html) clauses `ENABLE TRIGGER` and `ENABLE RULE` for more information.

PostgreSQL treats the settings `origin` and `local` the same internally. Third-party replication systems may use these two values for their internal purposes, for example using `local` to designate a session whose changes should not be replicated.

Since foreign keys are implemented as triggers, setting this parameter to `replica` also disables all foreign key checks, which can leave data in an inconsistent state if improperly used.
