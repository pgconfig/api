---
name: "wal_recycle"
version: "15"
type: "boolean"
category: "Write-Ahead Log / Settings"
short_desc: "Recycles WAL files by renaming them."
context: "superuser"
default: "on"
url: "https://www.postgresql.org/docs/15/runtime-config-wal.html#GUC-WAL-RECYCLE"
---

If set to `on` (the default), this option causes WAL files to be recycled by renaming them, avoiding the need to create new ones. On COW file systems, it may be faster to create new ones, so the option is given to disable this behavior.
