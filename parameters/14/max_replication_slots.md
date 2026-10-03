---
name: "max_replication_slots"
version: "14"
type: "integer"
category: "Replication / Sending Servers"
short_desc: "Sets the maximum number of simultaneously defined replication slots."
context: "postmaster"
default: "10"
min: "0"
max: "262143"
url: "https://www.postgresql.org/docs/14/runtime-config-replication.html#GUC-MAX-REPLICATION-SLOTS"
---

Specifies the maximum number of replication slots (see [Replication Slots](https://www.postgresql.org/docs/14/warm-standby.html#STREAMING-REPLICATION-SLOTS)) that the server can support. The default is 10. This parameter can only be set at server start. Setting it to a lower value than the number of currently existing replication slots will prevent the server from starting. Also, `wal_level` must be set to `replica` or higher to allow replication slots to be used.

On the subscriber side, specifies how many replication origins (see [Replication Progress Tracking](https://www.postgresql.org/docs/14/replication-origins.html)) can be tracked simultaneously, effectively limiting how many logical replication subscriptions can be created on the server. Setting it to a lower value than the current number of tracked replication origins (reflected in [pg_replication_origin_status](https://www.postgresql.org/docs/14/view-pg-replication-origin-status.html), not [pg_replication_origin](https://www.postgresql.org/docs/14/catalog-pg-replication-origin.html)) will prevent the server from starting.
