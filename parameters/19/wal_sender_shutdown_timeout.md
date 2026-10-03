---
name: "wal_sender_shutdown_timeout"
version: "19"
type: "integer"
category: "Replication / Sending Servers"
short_desc: "Sets the maximum time the server waits during shutdown for all WAL data to be replicated to the receiver."
extra_desc: "-1 means wait indefinitely. 0 means do not wait."
context: "user"
unit: "ms"
default: "-1"
min: "-1"
max: "2147483647"
url: "https://www.postgresql.org/docs/19/runtime-config-replication.html#GUC-WAL-SENDER-SHUTDOWN-TIMEOUT"
---

Specifies the maximum time the server waits during shutdown for all WAL data to be replicated to the receiver. If this value is specified without units, it is taken as milliseconds. A value of `-1` (the default) disables the timeout mechanism, allowing the WAL sender to wait as long as necessary for the receiver to catch up. A value of `0` causes the WAL sender to terminate without waiting for the receiver to catch up.

When replication is in use, the sending server normally waits until all WAL data has been transferred to the receiver before completing shutdown. This helps keep sender and receiver in sync after shutdown, which is especially important for physical replication switchovers, but it can delay shutdown.

If this parameter is set to zero or a positive value, the server stops waiting and completes shutdown when the timeout expires. This can shorten shutdown time, for example, when replication is slow on high-latency networks or when a logical replication apply worker is blocked waiting for locks. However, in this case the sender and receiver may be out of sync after shutdown. Care should be taken to select a value high enough to allow all WAL data to be replicated to the receiver under normal circumstances.

This parameter can be set in `primary_conninfo` and in the `CONNECTION` clause of `CREATE SUBSCRIPTION` (for example, include `options=-cwal_sender_shutdown_timeout=10s` in the connection string), allowing different timeouts per replication connection. For example, when both physical and logical replication are used, it can be disabled for physical replication (e.g., for switchovers) while enabled for logical replication to limit shutdown time.
