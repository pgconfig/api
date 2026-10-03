---
name: "max_wal_senders"
version: "11"
type: "integer"
category: "Replication / Sending Servers"
short_desc: "Sets the maximum number of simultaneously running WAL sender processes."
context: "postmaster"
default: "10"
min: "0"
max: "262143"
url: "https://www.postgresql.org/docs/11/runtime-config-replication.html#GUC-MAX-WAL-SENDERS"
---

Specifies the maximum number of concurrent connections from standby servers or streaming base backup clients (i.e., the maximum number of simultaneously running WAL sender processes). The default is 10. The value 0 means replication is disabled. WAL sender processes count towards the total number of connections, so this parameter's value must be less than [`max_connections`](https://www.postgresql.org/docs/11/runtime-config-connection.html#GUC-MAX-CONNECTIONS) minus [`superuser_reserved_connections`](https://www.postgresql.org/docs/11/runtime-config-connection.html#GUC-SUPERUSER-RESERVED-CONNECTIONS). Abrupt streaming client disconnection might leave an orphaned connection slot behind until a timeout is reached, so this parameter should be set slightly higher than the maximum number of expected clients so disconnected clients can immediately reconnect. This parameter can only be set at server start. Also, `wal_level` must be set to `replica` or higher to allow connections from standby servers.
