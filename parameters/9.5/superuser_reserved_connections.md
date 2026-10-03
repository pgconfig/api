---
name: "superuser_reserved_connections"
version: "9.5"
type: "integer"
category: "Connections and Authentication / Connection Settings"
short_desc: "Sets the number of connection slots reserved for superusers."
context: "postmaster"
default: "3"
min: "0"
max: "8388607"
url: "https://www.postgresql.org/docs/9.5/runtime-config-connection.html#GUC-SUPERUSER-RESERVED-CONNECTIONS"
---

Determines the number of connection "slots" that are reserved for connections by PostgreSQL superusers. At most [`max_connections`](https://www.postgresql.org/docs/9.5/runtime-config-connection.html#GUC-MAX-CONNECTIONS) connections can ever be active simultaneously. Whenever the number of active concurrent connections is at least `max_connections` minus `superuser_reserved_connections`, new connections will be accepted only for superusers, and no new replication connections will be accepted.

The default value is three connections. The value must be less than the value of `max_connections`. This parameter can only be set at server start.
