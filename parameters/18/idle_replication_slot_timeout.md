---
name: "idle_replication_slot_timeout"
version: "18"
type: "integer"
category: "Replication / Sending Servers"
short_desc: "Sets the duration a replication slot can remain idle before it is invalidated."
context: "sighup"
unit: "s"
default: "0"
min: "0"
max: "2147483647"
url: "https://www.postgresql.org/docs/18/runtime-config-replication.html#GUC-IDLE-REPLICATION-SLOT-TIMEOUT"
---

Invalidate replication slots that have remained inactive (not used by a [replication connection](https://www.postgresql.org/docs/18/protocol-replication.html)) for longer than this duration. If this value is specified without units, it is taken as seconds. A value of zero (the default) disables the idle timeout invalidation mechanism. This parameter can only be set in the `postgresql.conf` file or on the server command line.

Slot invalidation due to idle timeout occurs during checkpoint. Because checkpoints happen at `checkpoint_timeout` intervals, there can be some lag between when the `idle_replication_slot_timeout` was exceeded and when the slot invalidation is triggered at the next checkpoint. To avoid such lags, users can force a checkpoint to promptly invalidate inactive slots. The duration of slot inactivity is calculated using the slot's [pg_replication_slots](https://www.postgresql.org/docs/18/view-pg-replication-slots.html).`inactive_since` value.

Note that the idle timeout invalidation mechanism is not applicable for slots that do not reserve WAL or for slots on the standby server that are being synced from the primary server (i.e., standby slots having [pg_replication_slots](https://www.postgresql.org/docs/18/view-pg-replication-slots.html).`synced` value `true`). Synced slots are always considered to be inactive because they don't perform logical decoding to produce changes.
