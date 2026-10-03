---
name: "recovery_target_xid"
version: "17"
type: "string"
category: "Write-Ahead Log / Recovery Target"
short_desc: "Sets the transaction ID up to which recovery will proceed."
context: "postmaster"
default: ""
url: "https://www.postgresql.org/docs/17/runtime-config-wal.html#GUC-RECOVERY-TARGET-XID"
---

This parameter specifies the transaction ID up to which recovery will proceed. Keep in mind that while transaction IDs are assigned sequentially at transaction start, transactions can complete in a different numeric order. The transactions that will be recovered are those that committed before (and optionally including) the specified one. The precise stopping point is also influenced by [`recovery_target_inclusive`](https://www.postgresql.org/docs/17/runtime-config-wal.html#GUC-RECOVERY-TARGET-INCLUSIVE).
