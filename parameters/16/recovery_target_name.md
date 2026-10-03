---
name: "recovery_target_name"
version: "16"
type: "string"
category: "Write-Ahead Log / Recovery Target"
short_desc: "Sets the named restore point up to which recovery will proceed."
context: "postmaster"
default: ""
url: "https://www.postgresql.org/docs/16/runtime-config-wal.html#GUC-RECOVERY-TARGET-NAME"
---

This parameter specifies the named restore point (created with `pg_create_restore_point()`) to which recovery will proceed.
