---
name: "serializable_buffers"
version: "19"
type: "integer"
category: "Resource Usage / Memory"
short_desc: "Sets the size of the dedicated buffer pool used for the serializable transaction cache."
context: "postmaster"
unit: "8kB"
default: "32"
min: "16"
max: "131072"
url: "https://www.postgresql.org/docs/19/runtime-config-resource.html#GUC-SERIALIZABLE-BUFFERS"
---

Specifies the amount of shared memory to use to cache the contents of `pg_serial` (see [Contents of PGDATA](https://www.postgresql.org/docs/19/storage-file-layout.html#PGDATA-CONTENTS-TABLE)). If this value is specified without units, it is taken as blocks, that is `BLCKSZ` bytes, typically 8kB. The default value is `32`. This parameter can only be set at server start.
