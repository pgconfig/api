---
name: "recovery_target"
version: "17"
type: "string"
category: "Write-Ahead Log / Recovery Target"
short_desc: "Set to \"immediate\" to end recovery as soon as a consistent state is reached."
context: "postmaster"
default: ""
url: "https://www.postgresql.org/docs/17/runtime-config-wal.html#GUC-RECOVERY-TARGET"
---

This parameter specifies that recovery should end as soon as a consistent state is reached, i.e., as early as possible. When restoring from an online backup, this means the point where taking the backup ended.

Technically, this is a string parameter, but `'immediate'` is currently the only allowed value.
