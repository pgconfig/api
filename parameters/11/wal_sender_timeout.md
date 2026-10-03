---
name: "wal_sender_timeout"
version: "11"
type: "integer"
category: "Replication / Sending Servers"
short_desc: "Sets the maximum time to wait for WAL replication."
context: "sighup"
unit: "ms"
default: "60000"
min: "0"
max: "2147483647"
url: "https://www.postgresql.org/docs/11/runtime-config-replication.html#GUC-WAL-SENDER-TIMEOUT"
---

Terminate replication connections that are inactive longer than the specified number of milliseconds. This is useful for the sending server to detect a standby crash or network outage. A value of zero disables the timeout mechanism. This parameter can only be set in the `postgresql.conf` file or on the server command line. The default value is 60 seconds.
