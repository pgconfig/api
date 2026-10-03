---
name: "wal_segment_size"
version: "9.5"
type: "integer"
category: "Preset Options"
short_desc: "Shows the number of pages per write ahead log segment."
context: "internal"
unit: "8kB"
default: "2048"
min: "2048"
max: "2048"
url: "https://www.postgresql.org/docs/9.5/runtime-config-preset.html#GUC-WAL-SEGMENT-SIZE"
---

Reports the number of blocks (pages) in a WAL segment file. The total size of a WAL segment file in bytes is equal to `wal_segment_size` multiplied by `wal_block_size`; by default this is 16MB. See [WAL Configuration](https://www.postgresql.org/docs/9.5/wal-configuration.html) for more information.
