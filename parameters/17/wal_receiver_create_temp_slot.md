---
name: "wal_receiver_create_temp_slot"
version: "17"
type: "boolean"
category: "Replication / Standby Servers"
short_desc: "Sets whether a WAL receiver should create a temporary replication slot if no permanent slot is configured."
context: "sighup"
default: "off"
url: "https://www.postgresql.org/docs/17/runtime-config-replication.html#GUC-WAL-RECEIVER-CREATE-TEMP-SLOT"
---

Specifies whether the WAL receiver process should create a temporary replication slot on the remote instance when no permanent replication slot to use has been configured (using [`primary_slot_name`](https://www.postgresql.org/docs/17/runtime-config-replication.html#GUC-PRIMARY-SLOT-NAME)). The default is off. This parameter can only be set in the `postgresql.conf` file or on the server command line. If this parameter is changed while the WAL receiver process is running, that process is signaled to shut down and expected to restart with the new setting.
