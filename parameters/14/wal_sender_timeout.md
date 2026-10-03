---
name: "wal_sender_timeout"
version: "14"
type: "integer"
category: "Replication / Sending Servers"
short_desc: "Sets the maximum time to wait for WAL replication."
context: "user"
unit: "ms"
default: "60000"
min: "0"
max: "2147483647"
url: "https://www.postgresql.org/docs/14/runtime-config-replication.html#GUC-WAL-SENDER-TIMEOUT"
---

Terminate replication connections that are inactive for longer than this amount of time. This is useful for the sending server to detect a standby crash or network outage. If this value is specified without units, it is taken as milliseconds. The default value is 60 seconds. A value of zero disables the timeout mechanism.

With a cluster distributed across multiple geographic locations, using different values per location brings more flexibility in the cluster management. A smaller value is useful for faster failure detection with a standby having a low-latency network connection, and a larger value helps in judging better the health of a standby if located on a remote location, with a high-latency network connection.
