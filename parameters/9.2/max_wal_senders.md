---
name: "max_wal_senders"
version: "9.2"
type: "integer"
category: "Replication / Sending Servers"
short_desc: "Sets the maximum number of simultaneously running WAL sender processes."
context: "postmaster"
default: "0"
min: "0"
max: "8388607"
url: "https://www.postgresql.org/docs/9.2/runtime-config-replication.html#GUC-MAX-WAL-SENDERS"
---

Specifies the maximum number of concurrent connections from standby servers or streaming base backup clients (i.e., the maximum number of simultaneously running WAL sender processes). The default is zero, meaning replication is disabled. WAL sender processes count towards the total number of connections, so the parameter cannot be set higher than [`max_connections`](https://www.postgresql.org/docs/9.2/runtime-config-connection.html#GUC-MAX-CONNECTIONS). This parameter can only be set at server start. `wal_level` must be set to `archive` or `hot_standby` to allow connections from standby servers.
