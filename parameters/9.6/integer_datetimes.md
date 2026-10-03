---
name: "integer_datetimes"
version: "9.6"
type: "boolean"
category: "Preset Options"
short_desc: "Datetimes are integer based."
context: "internal"
default: "on"
url: "https://www.postgresql.org/docs/9.6/runtime-config-preset.html#GUC-INTEGER-DATETIMES"
---

Reports whether PostgreSQL was built with support for 64-bit-integer dates and times. This can be disabled by configuring with `--disable-integer-datetimes` when building PostgreSQL. The default value is `on`.
