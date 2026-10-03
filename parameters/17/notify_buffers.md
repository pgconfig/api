---
name: "notify_buffers"
version: "17"
type: "integer"
category: "Resource Usage / Memory"
short_desc: "Sets the size of the dedicated buffer pool used for the LISTEN/NOTIFY message cache."
context: "postmaster"
unit: "8kB"
default: "16"
min: "16"
max: "131072"
url: "https://www.postgresql.org/docs/17/runtime-config-resource.html#GUC-NOTIFY-BUFFERS"
---

Specifies the amount of shared memory to use to cache the contents of `pg_notify` (see [Contents of PGDATA](https://www.postgresql.org/docs/17/storage-file-layout.html#PGDATA-CONTENTS-TABLE)). If this value is specified without units, it is taken as blocks, that is `BLCKSZ` bytes, typically 8kB. The default value is `16`. This parameter can only be set at server start.
