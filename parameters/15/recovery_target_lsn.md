---
name: "recovery_target_lsn"
version: "15"
type: "string"
category: "Write-Ahead Log / Recovery Target"
short_desc: "Sets the LSN of the write-ahead log location up to which recovery will proceed."
context: "postmaster"
default: ""
url: "https://www.postgresql.org/docs/15/runtime-config-wal.html#GUC-RECOVERY-TARGET-LSN"
---

This parameter specifies the LSN of the write-ahead log location up to which recovery will proceed. The precise stopping point is also influenced by [`recovery_target_inclusive`](https://www.postgresql.org/docs/15/runtime-config-wal.html#GUC-RECOVERY-TARGET-INCLUSIVE). This parameter is parsed using the system data type [`pg_lsn`](https://www.postgresql.org/docs/15/datatype-pg-lsn.html).
