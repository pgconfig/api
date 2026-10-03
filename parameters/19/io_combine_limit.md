---
name: "io_combine_limit"
version: "19"
type: "integer"
category: "Resource Usage / I/O"
short_desc: "Limit on the size of data reads and writes."
context: "user"
unit: "8kB"
default: "16"
min: "1"
max: "128"
url: "https://www.postgresql.org/docs/19/runtime-config-resource.html#GUC-IO-COMBINE-LIMIT"
---

Controls the largest I/O size in operations that combine I/O. If set higher than the `io_max_combine_limit` parameter, the lower value will silently be used instead, so both may need to be raised to increase the I/O size. If this value is specified without units, it is taken as blocks, that is `BLCKSZ` bytes, typically 8kB. The maximum possible size depends on the operating system and block size, but is typically 1MB on Unix and 128kB on Windows. The default is 128kB.
