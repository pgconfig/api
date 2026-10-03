---
name: "max_wal_senders"
version: "12"
type: "integer"
category: "Replication / Sending Servers"
short_desc: "Sets the maximum number of simultaneously running WAL sender processes."
context: "postmaster"
default: "10"
min: "0"
max: "262143"
url: "https://www.postgresql.org/docs/12/runtime-config-replication.html#GUC-MAX-WAL-SENDERS"
---

Specifies the maximum number of concurrent connections from standby servers or streaming base backup clients (i.e., the maximum number of simultaneously running WAL sender processes). The default is `10`. The value `0` means replication is disabled. Abrupt streaming client disconnection might leave an orphaned connection slot behind until a timeout is reached, so this parameter should be set slightly higher than the maximum number of expected clients so disconnected clients can immediately reconnect. This parameter can only be set at server start. Also, `wal_level` must be set to `replica` or higher to allow connections from standby servers.

When running a standby server, you must set this parameter to the same or higher value than on the master server. Otherwise, queries will not be allowed in the standby server.
