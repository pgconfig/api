---
name: "wal_decode_buffer_size"
version: "19"
type: "integer"
category: "Write-Ahead Log / Recovery"
short_desc: "Buffer size for reading ahead in the WAL during recovery."
extra_desc: "Maximum distance to read ahead in the WAL to prefetch referenced data blocks."
context: "postmaster"
unit: "B"
default: "524288"
min: "65536"
max: "1073741823"
url: "https://www.postgresql.org/docs/19/runtime-config-wal.html#GUC-WAL-DECODE-BUFFER-SIZE"
---

A limit on how far ahead the server can look in the WAL, to find blocks to prefetch. If this value is specified without units, it is taken as bytes. The default is 512kB. This parameter can only be set at server start.
