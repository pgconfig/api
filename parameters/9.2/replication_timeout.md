---
name: "replication_timeout"
version: "9.2"
type: "integer"
category: "Replication / Sending Servers"
short_desc: "Sets the maximum time to wait for WAL replication."
context: "sighup"
unit: "ms"
default: "60000"
min: "0"
max: "2147483647"
url: "https://www.postgresql.org/docs/9.2/runtime-config-replication.html#GUC-REPLICATION-TIMEOUT"
---

Terminate replication connections that are inactive longer than the specified number of milliseconds. This is useful for the sending server to detect a standby crash or network outage. A value of zero disables the timeout mechanism. This parameter can only be set in the `postgresql.conf` file or on the server command line. The default value is 60 seconds.

To prevent connections from being terminated prematurely, [`wal_receiver_status_interval`](https://www.postgresql.org/docs/9.2/runtime-config-replication.html#GUC-WAL-RECEIVER-STATUS-INTERVAL) must be enabled on the standby, and its value must be less than the value of `replication_timeout`.
