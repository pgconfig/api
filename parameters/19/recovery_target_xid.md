---
name: "recovery_target_xid"
version: "19"
type: "string"
category: "Write-Ahead Log / Recovery Target"
short_desc: "Sets the transaction ID up to which recovery will proceed."
context: "postmaster"
default: ""
url: "https://www.postgresql.org/docs/19/runtime-config-wal.html#GUC-RECOVERY-TARGET-XID"
---

This parameter specifies the transaction ID up to which recovery will proceed. Keep in mind that while transaction IDs are assigned sequentially at transaction start, transactions can complete in a different numeric order. The transactions that will be recovered are those that committed before (and optionally including) the specified one. The precise stopping point is also influenced by [`recovery_target_inclusive`](https://www.postgresql.org/docs/19/runtime-config-wal.html#GUC-RECOVERY-TARGET-INCLUSIVE).

The value can be specified as either a 32-bit transaction ID or a 64-bit transaction ID (consisting of an epoch and a 32-bit ID), such as the value returned by `pg_current_xact_id()`. When a 64-bit transaction ID is provided, only its 32-bit transaction ID portion is used as the recovery target. For example, the values 4294968296 (epoch 1) and 8589935592 (epoch 2) both refer to the same 32-bit transaction ID, 1000.

The effective transaction ID (the 32-bit portion) must be greater than or equal to 3.
