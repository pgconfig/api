---
name: "wal_init_zero"
version: "16"
type: "boolean"
category: "Write-Ahead Log / Settings"
short_desc: "Writes zeroes to new WAL files before first use."
context: "superuser"
default: "on"
url: "https://www.postgresql.org/docs/16/runtime-config-wal.html#GUC-WAL-INIT-ZERO"
---

If set to `on` (the default), this option causes new WAL files to be filled with zeroes. On some file systems, this ensures that space is allocated before we need to write WAL records. However, *Copy-On-Write* (COW) file systems may not benefit from this technique, so the option is given to skip the unnecessary work. If set to `off`, only the final byte is written when the file is created so that it has the expected size.
