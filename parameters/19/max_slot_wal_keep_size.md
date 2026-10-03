---
name: "max_slot_wal_keep_size"
version: "19"
type: "integer"
category: "Replication / Sending Servers"
short_desc: "Sets the maximum WAL size that can be reserved by replication slots."
extra_desc: "Replication slots will be marked as failed, and segments released for deletion or recycling, if this much space is occupied by WAL on disk. -1 means no maximum."
context: "sighup"
unit: "MB"
default: "-1"
min: "-1"
max: "2147483647"
url: "https://www.postgresql.org/docs/19/runtime-config-replication.html#GUC-MAX-SLOT-WAL-KEEP-SIZE"
---

Specify the maximum size of WAL files that [replication slots](https://www.postgresql.org/docs/19/warm-standby.html#STREAMING-REPLICATION-SLOTS) are allowed to retain in the `pg_wal` directory at checkpoint time. If `max_slot_wal_keep_size` is -1 (the default), replication slots may retain an unlimited amount of WAL files. Otherwise, if restart_lsn of a replication slot falls behind the current LSN by more than the given size, the standby using the slot may no longer be able to continue replication due to removal of required WAL files. You can see the WAL availability of replication slots in [pg_replication_slots](https://www.postgresql.org/docs/19/view-pg-replication-slots.html). If this value is specified without units, it is taken as megabytes. This parameter can only be set in the `postgresql.conf` file or on the server command line.
