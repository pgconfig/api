---
name: "wal_block_size"
version: "14"
type: "integer"
category: "Preset Options"
short_desc: "Shows the block size in the write ahead log."
context: "internal"
default: "8192"
min: "8192"
max: "8192"
url: "https://www.postgresql.org/docs/14/runtime-config-preset.html#GUC-WAL-BLOCK-SIZE"
---

Reports the size of a WAL disk block. It is determined by the value of `XLOG_BLCKSZ` when building the server. The default value is 8192 bytes.
