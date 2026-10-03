---
name: "primary_slot_name"
version: "13"
type: "string"
category: "Replication / Standby Servers"
short_desc: "Sets the name of the replication slot to use on the sending server."
context: "sighup"
default: ""
url: "https://www.postgresql.org/docs/13/runtime-config-replication.html#GUC-PRIMARY-SLOT-NAME"
---

Optionally specifies an existing replication slot to be used when connecting to the sending server via streaming replication to control resource removal on the upstream node (see [Replication Slots](https://www.postgresql.org/docs/13/warm-standby.html#STREAMING-REPLICATION-SLOTS)). This parameter can only be set in the `postgresql.conf` file or on the server command line. If this parameter is changed while the WAL receiver process is running, that process is signaled to shut down and expected to restart with the new setting. This setting has no effect if `primary_conninfo` is not set or the server is not in standby mode.
