---
name: "io_combine_limit"
version: "17"
type: "integer"
category: "Resource Usage / Asynchronous Behavior"
short_desc: "Limit on the size of data reads and writes."
context: "user"
unit: "8kB"
default: "16"
min: "1"
max: "32"
url: "https://www.postgresql.org/docs/17/runtime-config-resource.html#GUC-IO-COMBINE-LIMIT"
---

Controls the largest I/O size in operations that combine I/O. If this value is specified without units, it is taken as blocks, that is `BLCKSZ` bytes, typically 8kB. The default is 128kB.
