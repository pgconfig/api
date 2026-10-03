---
name: "recovery_target_inclusive"
version: "16"
type: "boolean"
category: "Write-Ahead Log / Recovery Target"
short_desc: "Sets whether to include or exclude transaction with recovery target."
context: "postmaster"
default: "on"
url: "https://www.postgresql.org/docs/16/runtime-config-wal.html#GUC-RECOVERY-TARGET-INCLUSIVE"
---

Specifies whether to stop just after the specified recovery target (`on`), or just before the recovery target (`off`). Applies when [`recovery_target_lsn`](https://www.postgresql.org/docs/16/runtime-config-wal.html#GUC-RECOVERY-TARGET-LSN), [`recovery_target_time`](https://www.postgresql.org/docs/16/runtime-config-wal.html#GUC-RECOVERY-TARGET-TIME), or [`recovery_target_xid`](https://www.postgresql.org/docs/16/runtime-config-wal.html#GUC-RECOVERY-TARGET-XID) is specified. This setting controls whether transactions having exactly the target WAL location (LSN), commit time, or transaction ID, respectively, will be included in the recovery. Default is `on`.
