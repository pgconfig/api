---
name: "synchronized_standby_slots"
version: "18"
type: "string"
category: "Replication / Primary Server"
short_desc: "Lists streaming replication standby server replication slot names that logical WAL sender processes will wait for."
extra_desc: "Logical WAL sender processes will send decoded changes to output plugins only after the specified replication slots have confirmed receiving WAL."
context: "sighup"
default: ""
url: "https://www.postgresql.org/docs/18/runtime-config-replication.html#GUC-SYNCHRONIZED-STANDBY-SLOTS"
---

A comma-separated list of streaming replication standby server slot names that logical WAL sender processes will wait for. Logical WAL sender processes will send decoded changes to plugins only after the specified replication slots confirm receiving WAL. This guarantees that logical replication failover slots do not consume changes until those changes are received and flushed to corresponding physical standbys. If a logical replication connection is meant to switch to a physical standby after the standby is promoted, the physical replication slot for the standby should be listed here. Note that logical replication will not proceed if the slots specified in the `synchronized_standby_slots` do not exist or are invalidated. Additionally, the replication management functions [`pg_replication_slot_advance`](https://www.postgresql.org/docs/18/functions-admin.html#PG-REPLICATION-SLOT-ADVANCE), [`pg_logical_slot_get_changes`](https://www.postgresql.org/docs/18/functions-admin.html#PG-LOGICAL-SLOT-GET-CHANGES), and [`pg_logical_slot_peek_changes`](https://www.postgresql.org/docs/18/functions-admin.html#PG-LOGICAL-SLOT-PEEK-CHANGES), when used with logical failover slots, will block until all physical slots specified in `synchronized_standby_slots` have confirmed WAL receipt.

The standbys corresponding to the physical replication slots in `synchronized_standby_slots` must configure `sync_replication_slots = true` so they can receive logical failover slot changes from the primary.
