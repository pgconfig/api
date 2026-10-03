---
name: "max_repack_replication_slots"
version: "19"
type: "integer"
category: "Replication / Sending Servers"
short_desc: "Sets the maximum number of replication slots for use by REPACK."
context: "postmaster"
default: "5"
min: "0"
max: "262143"
url: "https://www.postgresql.org/docs/19/runtime-config-replication.html#GUC-MAX-REPACK-REPLICATION-SLOTS"
---

Specifies the maximum number of replication slots for use of the `REPACK` command. The default is 5. This parameter can only be set at server start.
