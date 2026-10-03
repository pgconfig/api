---
name: "wal_receiver_timeout"
version: "18"
type: "integer"
category: "Replication / Standby Servers"
short_desc: "Sets the maximum wait time to receive data from the sending server."
extra_desc: "0 disables the timeout."
context: "sighup"
unit: "ms"
default: "60000"
min: "0"
max: "2147483647"
url: "https://www.postgresql.org/docs/18/runtime-config-replication.html#GUC-WAL-RECEIVER-TIMEOUT"
---

Terminate replication connections that are inactive for longer than this amount of time. This is useful for the receiving standby server to detect a primary node crash or network outage. If this value is specified without units, it is taken as milliseconds. The default value is 60 seconds. A value of zero disables the timeout mechanism. This parameter can only be set in the `postgresql.conf` file or on the server command line.
