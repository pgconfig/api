---
name: "segment_size"
version: "19"
type: "integer"
category: "Preset Options"
short_desc: "Shows the number of pages per disk file."
context: "internal"
unit: "8kB"
default: "131072"
min: "131072"
max: "131072"
url: "https://www.postgresql.org/docs/19/runtime-config-preset.html#GUC-SEGMENT-SIZE"
---

Reports the number of blocks (pages) that can be stored within a file segment. It is determined by the value of `RELSEG_SIZE` when building the server. The maximum size of a segment file in bytes is equal to `segment_size` multiplied by `block_size`; by default this is 1GB.
