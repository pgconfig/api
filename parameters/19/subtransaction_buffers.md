---
name: "subtransaction_buffers"
version: "19"
type: "integer"
category: "Resource Usage / Memory"
short_desc: "Sets the size of the dedicated buffer pool used for the subtransaction cache."
extra_desc: "0 means use a fraction of \"shared_buffers\"."
context: "postmaster"
unit: "8kB"
default: "0"
min: "0"
max: "131072"
url: "https://www.postgresql.org/docs/19/runtime-config-resource.html#GUC-SUBTRANSACTION-BUFFERS"
---

Specifies the amount of shared memory to use to cache the contents of `pg_subtrans` (see [Contents of PGDATA](https://www.postgresql.org/docs/19/storage-file-layout.html#PGDATA-CONTENTS-TABLE)). If this value is specified without units, it is taken as blocks, that is `BLCKSZ` bytes, typically 8kB. The default value is `0`, which requests `shared_buffers`/512 up to 1024 blocks, but not fewer than 16 blocks. This parameter can only be set at server start.
