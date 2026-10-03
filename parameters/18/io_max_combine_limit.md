---
name: "io_max_combine_limit"
version: "18"
type: "integer"
category: "Resource Usage / I/O"
short_desc: "Server-wide limit that clamps io_combine_limit."
context: "postmaster"
unit: "8kB"
default: "16"
min: "1"
max: "128"
url: "https://www.postgresql.org/docs/18/runtime-config-resource.html#GUC-IO-MAX-COMBINE-LIMIT"
---

Controls the largest I/O size in operations that combine I/O, and silently limits the user-settable parameter `io_combine_limit`. This parameter can only be set at server start. If this value is specified without units, it is taken as blocks, that is `BLCKSZ` bytes, typically 8kB. The maximum possible size depends on the operating system and block size, but is typically 1MB on Unix and 128kB on Windows. The default is 128kB.
