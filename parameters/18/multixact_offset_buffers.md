---
name: "multixact_offset_buffers"
version: "18"
type: "integer"
category: "Resource Usage / Memory"
short_desc: "Sets the size of the dedicated buffer pool used for the MultiXact offset cache."
context: "postmaster"
unit: "8kB"
default: "16"
min: "16"
max: "131072"
url: "https://www.postgresql.org/docs/18/runtime-config-resource.html#GUC-MULTIXACT-OFFSET-BUFFERS"
---

Specifies the amount of shared memory to use to cache the contents of `pg_multixact/offsets` (see [Contents of PGDATA](https://www.postgresql.org/docs/18/storage-file-layout.html#PGDATA-CONTENTS-TABLE)). If this value is specified without units, it is taken as blocks, that is `BLCKSZ` bytes, typically 8kB. The default value is `16`. This parameter can only be set at server start.
