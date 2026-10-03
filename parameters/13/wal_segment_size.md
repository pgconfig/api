---
name: "wal_segment_size"
version: "13"
type: "integer"
category: "Preset Options"
short_desc: "Shows the size of write ahead log segments."
context: "internal"
unit: "B"
default: "16777216"
min: "1048576"
max: "1073741824"
url: "https://www.postgresql.org/docs/13/runtime-config-preset.html#GUC-WAL-SEGMENT-SIZE"
---

Reports the size of write ahead log segments. The default value is 16MB. See [WAL Configuration](https://www.postgresql.org/docs/13/wal-configuration.html) for more information.
