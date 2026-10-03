---
name: "max_replication_slots"
version: "17"
type: "integer"
category: "Replication / Sending Servers"
short_desc: "Sets the maximum number of simultaneously defined replication slots."
context: "postmaster"
default: "10"
min: "0"
max: "262143"
url: "https://www.postgresql.org/docs/17/runtime-config-replication.html#GUC-MAX-REPLICATION-SLOTS"
---

Specifies the maximum number of replication slots (see [Replication Slots](https://www.postgresql.org/docs/17/warm-standby.html#STREAMING-REPLICATION-SLOTS)) that the server can support. The default is 10. This parameter can only be set at server start. Setting it to a lower value than the number of currently existing replication slots will prevent the server from starting. Also, `wal_level` must be set to `replica` or higher to allow replication slots to be used.

Note that this parameter also applies on the subscriber side, but with a different meaning.

Specifies how many replication origins (see [Replication Progress Tracking](https://www.postgresql.org/docs/17/replication-origins.html)) can be tracked simultaneously, effectively limiting how many logical replication subscriptions can be created on the server. Setting it to a lower value than the current number of tracked replication origins (reflected in [pg_replication_origin_status](https://www.postgresql.org/docs/17/view-pg-replication-origin-status.html)) will prevent the server from starting. `max_replication_slots` must be set to at least the number of subscriptions that will be added to the subscriber, plus some reserve for table synchronization.

Note that this parameter also applies on a sending server, but with a different meaning.
