---
name: "wal_sender_delay"
version: "9.1"
type: "integer"
category: "Replication / Master Server"
short_desc: "WAL sender sleep time between WAL replications."
context: "sighup"
unit: "ms"
default: "1000"
min: "1"
max: "10000"
url: "https://www.postgresql.org/docs/9.1/runtime-config-replication.html#GUC-WAL-SENDER-DELAY"
---

Specifies the delay between activity rounds for WAL sender processes. In each round the WAL sender sends any WAL accumulated since the last round to the standby server. It then sleeps for `wal_sender_delay` milliseconds, and repeats. The sleep is interrupted by transaction commit, so the effects of a committed transaction are sent to standby servers as soon as the commit happens, regardless of this setting. The default value is one second (`1s`). Note that on many systems, the effective resolution of sleep delays is 10 milliseconds; setting `wal_sender_delay` to a value that is not a multiple of 10 might have the same results as setting it to the next higher multiple of 10. This parameter can only be set in the `postgresql.conf` file or on the server command line.
