---
name: "primary_slot_name"
version: "12"
type: "string"
category: "Replication / Standby Servers"
short_desc: "Sets the name of the replication slot to use on the sending server."
context: "postmaster"
default: ""
url: "https://www.postgresql.org/docs/12/runtime-config-replication.html#GUC-PRIMARY-SLOT-NAME"
---

Optionally specifies an existing replication slot to be used when connecting to the sending server via streaming replication to control resource removal on the upstream node (see [Replication Slots](https://www.postgresql.org/docs/12/warm-standby.html#STREAMING-REPLICATION-SLOTS)). This parameter can only be set at server start. This setting has no effect if `primary_conninfo` is not set.
