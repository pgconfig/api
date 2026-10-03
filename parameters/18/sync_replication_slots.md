---
name: "sync_replication_slots"
version: "18"
type: "boolean"
category: "Replication / Standby Servers"
short_desc: "Enables a physical standby to synchronize logical failover replication slots from the primary server."
context: "sighup"
default: "off"
url: "https://www.postgresql.org/docs/18/runtime-config-replication.html#GUC-SYNC-REPLICATION-SLOTS"
---

It enables a physical standby to synchronize logical failover slots from the primary server so that logical subscribers can resume replication from the new primary server after failover.

It is disabled by default. This parameter can only be set in the `postgresql.conf` file or on the server command line.
