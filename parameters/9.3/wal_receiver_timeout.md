---
name: "wal_receiver_timeout"
version: "9.3"
type: "integer"
category: "Replication / Standby Servers"
short_desc: "Sets the maximum wait time to receive data from the primary."
context: "sighup"
unit: "ms"
default: "60000"
min: "0"
max: "2147483647"
url: "https://www.postgresql.org/docs/9.3/runtime-config-replication.html#GUC-WAL-RECEIVER-TIMEOUT"
---

Terminate replication connections that are inactive longer than the specified number of milliseconds. This is useful for the receiving standby server to detect a primary node crash or network outage. A value of zero disables the timeout mechanism. This parameter can only be set in the `postgresql.conf` file or on the server command line. The default value is 60 seconds.
