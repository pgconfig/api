---
name: "max_replication_slots"
version: "9.6"
type: "integer"
category: "Replication / Sending Servers"
short_desc: "Sets the maximum number of simultaneously defined replication slots."
context: "postmaster"
default: "0"
min: "0"
max: "262143"
url: "https://www.postgresql.org/docs/9.6/runtime-config-replication.html#GUC-MAX-REPLICATION-SLOTS"
---

Specifies the maximum number of replication slots (see [Replication Slots](https://www.postgresql.org/docs/9.6/warm-standby.html#STREAMING-REPLICATION-SLOTS)) that the server can support. The default is zero. This parameter can only be set at server start. `wal_level` must be set to `replica` or higher to allow replication slots to be used. Setting it to a lower value than the number of currently existing replication slots will prevent the server from starting.
