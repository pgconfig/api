---
name: "max_active_replication_origins"
version: "18"
type: "integer"
category: "Replication / Subscribers"
short_desc: "Sets the maximum number of active replication origins."
context: "postmaster"
default: "10"
min: "0"
max: "262143"
url: "https://www.postgresql.org/docs/18/runtime-config-replication.html#GUC-MAX-ACTIVE-REPLICATION-ORIGINS"
---

Specifies how many replication origins (see [Replication Progress Tracking](https://www.postgresql.org/docs/18/replication-origins.html)) can be tracked simultaneously, effectively limiting how many logical replication subscriptions can be created on the server. Setting it to a lower value than the current number of tracked replication origins (reflected in [pg_replication_origin_status](https://www.postgresql.org/docs/18/view-pg-replication-origin-status.html)) will prevent the server from starting. It defaults to 10. This parameter can only be set at server start. `max_active_replication_origins` must be set to at least the number of subscriptions that will be added to the subscriber, plus some reserve for table synchronization.
