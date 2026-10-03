---
name: "superuser_reserved_connections"
version: "17"
type: "integer"
category: "Connections and Authentication / Connection Settings"
short_desc: "Sets the number of connection slots reserved for superusers."
context: "postmaster"
default: "3"
min: "0"
max: "262143"
url: "https://www.postgresql.org/docs/17/runtime-config-connection.html#GUC-SUPERUSER-RESERVED-CONNECTIONS"
---

Determines the number of connection "slots" that are reserved for connections by PostgreSQL superusers. At most [`max_connections`](https://www.postgresql.org/docs/17/runtime-config-connection.html#GUC-MAX-CONNECTIONS) connections can ever be active simultaneously. Whenever the number of active concurrent connections is at least `max_connections` minus `superuser_reserved_connections`, new connections will be accepted only for superusers. The connection slots reserved by this parameter are intended as final reserve for emergency use after the slots reserved by [`reserved_connections`](https://www.postgresql.org/docs/17/runtime-config-connection.html#GUC-RESERVED-CONNECTIONS) have been exhausted.

The default value is three connections. The value must be less than `max_connections` minus `reserved_connections`. This parameter can only be set at server start.
