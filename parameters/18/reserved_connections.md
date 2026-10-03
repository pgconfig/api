---
name: "reserved_connections"
version: "18"
type: "integer"
category: "Connections and Authentication / Connection Settings"
short_desc: "Sets the number of connection slots reserved for roles with privileges of pg_use_reserved_connections."
context: "postmaster"
default: "0"
min: "0"
max: "262143"
url: "https://www.postgresql.org/docs/18/runtime-config-connection.html#GUC-RESERVED-CONNECTIONS"
---

Determines the number of connection "slots" that are reserved for connections by roles with privileges of the [pg_use_reserved_connections](https://www.postgresql.org/docs/18/predefined-roles.html#PREDEFINED-ROLE-PG-USE-RESERVED-CONNECTIONS) role. Whenever the number of free connection slots is greater than [`superuser_reserved_connections`](https://www.postgresql.org/docs/18/runtime-config-connection.html#GUC-SUPERUSER-RESERVED-CONNECTIONS) but less than or equal to the sum of `superuser_reserved_connections` and `reserved_connections`, new connections will be accepted only for superusers and roles with privileges of `pg_use_reserved_connections`. If `superuser_reserved_connections` or fewer connection slots are available, new connections will be accepted only for superusers.

The default value is zero connections. The value must be less than `max_connections` minus `superuser_reserved_connections`. This parameter can only be set at server start.
